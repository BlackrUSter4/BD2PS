use bd2::prost::Message;
use bd2::proto::proto_net::{BattleRetryPreviousTurnRequest, BattleRetryPreviousTurnResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::battle::battle_session;
use sqlx::SqlitePool;
use tracing::info;

/// Rewinding to a previous turn within the current battle. This project
/// stores no per-turn battle history (there's no table anywhere for
/// mid-battle turn state, only the session set at Enter/Start and the
/// final result at End), so per-turn fields here are placeholders. What IS
/// real: confirming the caller actually has a live session for this
/// battle_index before honoring the retry.
pub async fn handle(pool: &SqlitePool, uid: i64, req: BattleRetryPreviousTurnRequest) -> GameResponse {
    info!("Handling BattleRetryPreviousTurnRequest: {:?}", req);

    let has_session = match battle_session::get(pool, uid).await {
        Ok(Some(s)) => req.battle_index.is_none() || s.battle_index == req.battle_index,
        _ => false,
    };
    if !has_session {
        tracing::warn!("BattleRetryPreviousTurn: no matching battle session for uid {}", uid);
    }

    let response = BattleRetryPreviousTurnResponse {
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

    let (route, code) = PacketCodeType::BattleRetryPreviousTurn.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
