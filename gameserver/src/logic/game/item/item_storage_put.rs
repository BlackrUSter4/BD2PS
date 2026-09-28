use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, ItemStoragePutRequest, ItemStoragePutResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::item::item_info as item_db;
use sqlx::SqlitePool;
use tracing::info;

/// Real move-to-storage: flips the owned item's real IsStorage flag.
pub async fn handle(pool: &SqlitePool, uid: i64, req: ItemStoragePutRequest) -> GameResponse {
    info!("Handling ItemStoragePutRequest: {:?}", req);

    let mut item_info = Vec::new();
    if let Some(inven_index) = req.item_info.as_ref().and_then(|i| i.inven_index) {
        let _ = item_db::set_storage_flag(pool, uid, inven_index, true).await;
        if let Ok(Some(row)) = item_db::get_by_inven_index(pool, uid, inven_index).await {
            item_info.push(ItemDbInfo {
                inven_index: row.inven_index,
                id: row.id,
                r#type: row.r#type,
                count: row.count,
                keep_flag: row.keep_flag,
                time_value: row.time_value,
                expiry_time: row.expiry_time,
                sort_id: row.sort_id,
                use_count: row.use_count,
                pictorialbook_info: None,
            });
        }
    }

    let response = ItemStoragePutResponse { item_info };

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

    let (route, code) = PacketCodeType::ItemStoragePut.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
