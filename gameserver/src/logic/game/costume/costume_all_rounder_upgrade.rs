use bd2::prost::Message;
use bd2::proto::proto_net::{
    CostumeAllRounderUpgradeRequest, CostumeAllRounderUpgradeResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

/// No "all-rounder upgrade product" master table was captured anywhere in this project (the
/// only costume-upgrade table found, CostumeGrowthTable, is used by the plain CostumeUpgrade
/// handler instead) — real item consumption is applied, but there's no catalog to validate
/// product_group_id/product_id against or grant a specific real upgrade effect from.
pub async fn handle(pool: &SqlitePool, uid: i64, req: CostumeAllRounderUpgradeRequest) -> GameResponse {
    info!("Handling CostumeAllRounderUpgradeRequest: {:?}", req);

    for item in &req.use_item_info {
        if let (Some(id), Some(count)) = (item.id, item.count) {
            let _ = item_info::consume(pool, uid, id, count).await;
        }
    }

    let response = CostumeAllRounderUpgradeResponse {};

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

    let (route, code) = PacketCodeType::CostumeAllRounderUpgrade.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
