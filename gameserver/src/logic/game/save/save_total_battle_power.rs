use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, SaveTotalBattlePowerRequest, SaveTotalBattlePowerResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::total::total_battle_power as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account highest-battle-power tracking against the previously-unused TotalBattlePower
/// table (was hardcoding 1560 for every account, and used the wrong PacketCodeType —
/// PackEventStoryInfo — as a copy-paste mistake; both fixed).
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: SaveTotalBattlePowerRequest,
) -> GameResponse {
    info!("Handling SaveTotalBattlePowerRequest: {:?}", req);

    let reported = req.total_battle_power.unwrap_or(0);
    let highest = db::upsert_highest(pool, uid, reported).await.unwrap_or(reported);

    let response = SaveTotalBattlePowerResponse {
        highest_total_battle_power: Some(highest),
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::SaveTotalBattlePower.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
