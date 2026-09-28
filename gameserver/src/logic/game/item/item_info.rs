use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, ItemInfoRequest, ItemInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::item::item_info::get_item_info;
use database::models::game::item::item_info::ItemInfo;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: ItemInfoRequest) -> GameResponse {
    info!("Handling ItemInfoRequest: {:?}", req);

    let rows = match get_item_info(pool, uid).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Failed to fetch ItemInfo for uid {}: {}", uid, e);
            vec![]
        }
    };

    let item_info = rows
        .into_iter()
        .filter(|row| row.is_storage == 0)
        .map(|row: ItemInfo| ItemDbInfo {
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
        })
        .collect::<Vec<_>>();

    let response = ItemInfoResponse {
        item_info,
        ..Default::default()
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

    let (route, code) = PacketCodeType::ItemInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
