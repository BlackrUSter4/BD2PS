use bd2::prost::Message;
use bd2::proto::proto_net::{BattleVerifyStateRequest, BattleVerifyStateResponse, DefineBattleVerifyState, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::battle::battle_session;
use sqlx::SqlitePool;
use tracing::info;

/// Real bug found live (2026-09-30): this used to return the bare integer 1
/// (from a wrong guess that `state` was just a live-session boolean) any
/// time a BattleSession row existed, and never anything else. But the real
/// proto enum (`DefineBattleVerifyState`) has 4 values -- NONE/IN_PROGRESS/
/// SUCCESS/ERROR -- and the client polls this endpoint in a loop waiting
/// for it to leave IN_PROGRESS. Since nothing ever moved it off 1, the
/// client polled forever and eventually hit its OWN hardcoded retry cap,
/// surfacing as a client-side "전투검증 요청 횟수가 초과되었습니다." (battle
/// verification request count exceeded) error popup right after the
/// battle-result screen -- confirmed live via Player.log once
/// BD2CompatPatch was extended to log the popup's real title text.
/// This project has no real server-side battle simulation to verify the
/// client's report against (same trust-the-client model as every other
/// async battle system here: Colosseum/Ib/etc.), so there is nothing to
/// actually keep the client waiting on -- report SUCCESS immediately.
pub async fn handle(pool: &SqlitePool, uid: i64, req: BattleVerifyStateRequest) -> GameResponse {
    info!("Handling BattleVerifyStateRequest: {:?}", req);

    // Still a real read (not hardcoded): only claim success if this account
    // actually has a battle session on file; otherwise report NONE, which
    // is honest -- there is nothing here to verify.
    let state = match battle_session::get(pool, uid).await {
        Ok(Some(_)) => DefineBattleVerifyState::BattleVerifyStateSuccess as i32,
        _ => DefineBattleVerifyState::BattleVerifyStateNone as i32,
    };

    let response = BattleVerifyStateResponse {
        state: Some(state),
        ..Default::default()
    };
    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::BattleVerifyState.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
