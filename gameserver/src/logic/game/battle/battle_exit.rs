use bd2::prost::Message;
use bd2::proto::proto_net::{BattleExitRequest, BattleExitResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::battle::battle_session;
use sqlx::SqlitePool;
use tracing::info;

/// Leaving the battle screen without a result (e.g. backed out before
/// starting) — real job is just dropping the in-progress session.
pub async fn handle(pool: &SqlitePool, uid: i64, req: BattleExitRequest) -> GameResponse {
    info!("Handling BattleExitRequest: {:?}", req);

    if let Err(e) = battle_session::delete(pool, uid).await {
        tracing::warn!("BattleExit: failed to clear battle session: {}", e);
    }

    let response = BattleExitResponse {};
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

    let (route, code) = PacketCodeType::BattleExit.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
