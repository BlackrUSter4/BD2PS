use bd2::prost::Message;
use bd2::proto::proto_net::{ItemStorageUpdateRequest, ItemStorageUpdateResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::item::item_info as item_db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-item sort-order update for items already in storage (e.g. after the player
/// reorders their storage view client-side).
pub async fn handle(pool: &SqlitePool, uid: i64, req: ItemStorageUpdateRequest) -> GameResponse {
    info!("Handling ItemStorageUpdateRequest: {:?}", req);

    for item in &req.item_info {
        if let (Some(inven_index), Some(sort_id)) = (item.inven_index, item.sort_id) {
            let _ = item_db::set_sort_id(pool, uid, inven_index, sort_id).await;
        }
    }

    let response = ItemStorageUpdateResponse {};

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

    let (route, code) = PacketCodeType::ItemStorageUpdate.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
