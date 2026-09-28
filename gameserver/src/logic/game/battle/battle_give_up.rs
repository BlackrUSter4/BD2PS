use bd2::prost::Message;
use bd2::proto::proto_net::{BattleGiveUpRequest, BattleGiveUpResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::battle::battle_session;
use sqlx::SqlitePool;
use tracing::info;

/// Surrendering mid-battle — no reward, just a real session clear (same as
/// a loss, but the client already knows the outcome so there's nothing to
/// report back beyond acknowledging it). No user/monster-hunt table is
/// unambiguous enough here to justify populating `user_info` for real
/// beyond leaving it empty — no reward or currency change happens on a
/// give-up, so there's nothing that actually needs reflecting.
pub async fn handle(pool: &SqlitePool, uid: i64, req: BattleGiveUpRequest) -> GameResponse {
    info!("Handling BattleGiveUpRequest: {:?}", req);

    if let Err(e) = battle_session::delete(pool, uid).await {
        tracing::warn!("BattleGiveUp: failed to clear battle session: {}", e);
    }

    let response = BattleGiveUpResponse {
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

    let (route, code) = PacketCodeType::BattleGiveUp.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
