use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDumpRequest, ItemDumpResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::item::item_info as item_db;
use sqlx::SqlitePool;
use tracing::info;

/// Real item discard: reduces (or fully removes) the owned stack by the requested count.
pub async fn handle(pool: &SqlitePool, uid: i64, req: ItemDumpRequest) -> GameResponse {
    info!("Handling ItemDumpRequest: {:?}", req);

    if let Some(item) = &req.item_info {
        if let (Some(inven_index), Some(count)) = (item.inven_index, item.count) {
            let _ = item_db::reduce_by_inven_index(pool, uid, inven_index, count).await;
        }
    }

    let response = ItemDumpResponse {};

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

    let (route, code) = PacketCodeType::ItemDump.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
