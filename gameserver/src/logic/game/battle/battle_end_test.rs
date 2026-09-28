use bd2::prost::Message;
use bd2::proto::proto_net::{BattleEndTestRequest, BattleEndTestResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::battle::battle_session;
use sqlx::SqlitePool;
use tracing::info;

/// "Test" battle end — a sandbox/preview variant of BattleEnd (e.g. trying
/// a team composition). Judgment call: unlike a real BattleEnd, this does
/// NOT consume items or grant rewards (a test run costing real resources
/// or paying out real gold would be an obvious exploit) — it just reports
/// the outcome and clears the session so state doesn't leak into a real
/// subsequent battle.
pub async fn handle(pool: &SqlitePool, uid: i64, req: BattleEndTestRequest) -> GameResponse {
    info!("Handling BattleEndTestRequest: {:?}", req);

    if let Err(e) = battle_session::delete(pool, uid).await {
        tracing::warn!("BattleEndTest: failed to clear battle session: {}", e);
    }

    let response = BattleEndTestResponse {
        battle_result: req.battle_result,
        char_info: req.char_info.clone(),
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

    let (route, code) = PacketCodeType::BattleEndTest.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
