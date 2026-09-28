use super::now_ms;
use bd2::prost::Message;
use bd2::proto::proto_net::{FishingBoatUpgradeRequest, FishingBoatUpgradeResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// No payment field on the request and no `FishingBoatDesignTable`/level-cost data captured,
/// so the level is just incremented by 1 with a placeholder cap (10 — no real max known,
/// flagged) and no cost enforced.
const PLACEHOLDER_MAX_BOAT_LEVEL: i32 = 10;

pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingBoatUpgradeRequest) -> GameResponse {
    info!("Handling FishingBoatUpgradeRequest: {:?}", req);

    let user = database::db::fishing::fishing_user_info::get_or_create(pool, uid)
        .await
        .unwrap_or_default();
    let new_level = (user.boat_level + 1).min(PLACEHOLDER_MAX_BOAT_LEVEL);
    let _ = database::db::fishing::fishing_user_info::set_boat_level(pool, uid, new_level).await;

    let trap_time = user.trap_reward_receipt_time.unwrap_or_else(now_ms);
    if user.trap_reward_receipt_time.is_none() {
        let _ = database::db::fishing::fishing_user_info::set_trap_reward_receipt_time(pool, uid, trap_time).await;
    }

    let response = FishingBoatUpgradeResponse {
        level: Some(new_level),
        trap_reward_receipt_time: Some(trap_time),
        reward_info: None,
    };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::FishingBoatUpgrade.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
