use super::{fish_row_to_dbinfo, rod_row_to_dbinfo};
use bd2::prost::Message;
use bd2::proto::proto_net::{FishingItemDbInfo, FishingItemInfoRequest, FishingItemInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// First-ever call grants a starter rod (rod_id = 1 — no `FishingRodTable` was captured to
/// look up a real starter id from, flagged placeholder) so casting has something to equip.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingItemInfoRequest) -> GameResponse {
    info!("Handling FishingItemInfoRequest: {:?}", req);

    let mut rods = database::db::fishing::fishing_rod_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();
    if rods.is_empty() {
        if let Ok(idx) = database::db::fishing::fishing_rod_info::insert(pool, uid, 1, None).await {
            let _ = database::db::fishing::fishing_user_info::set_use_rod(pool, uid, idx).await;
            rods = database::db::fishing::fishing_rod_info::get_by_uid(pool, uid)
                .await
                .unwrap_or_default();
        }
    }
    let fish = database::db::fishing::fishing_fish_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();
    let items = database::db::fishing::fishing_item_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();

    let response = FishingItemInfoResponse {
        fish_info: fish.iter().map(fish_row_to_dbinfo).collect(),
        rod_info: rods.iter().map(rod_row_to_dbinfo).collect(),
        item_info: items
            .iter()
            .map(|r| FishingItemDbInfo {
                inven_index: Some(r.inven_index),
                id: Some(r.item_id),
                r#type: Some(r.item_type),
                count: Some(r.count),
                time_value: r.time_value,
            })
            .collect(),
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
    let (route, code) = PacketCodeType::FishingItemInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
