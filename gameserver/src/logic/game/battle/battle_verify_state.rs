use bd2::prost::Message;
use bd2::proto::proto_net::{BattleVerifyStateRequest, BattleVerifyStateResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::battle::battle_session;
use sqlx::SqlitePool;
use tracing::info;

/// Real check: does this account actually have a live battle session right
/// now (state=1) or not (state=0)? This is exactly what BattleSession
/// exists to answer. Red/blue rosters are left empty — BattleCharInfo has
/// no team-side column to split them by, so populating one arbitrarily
/// would be a fabrication rather than a real read.
pub async fn handle(pool: &SqlitePool, uid: i64, req: BattleVerifyStateRequest) -> GameResponse {
    info!("Handling BattleVerifyStateRequest: {:?}", req);

    let state = match battle_session::get(pool, uid).await {
        Ok(Some(_)) => 1,
        _ => 0,
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
