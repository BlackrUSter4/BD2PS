use super::user_to_dbinfo;
use bd2::prost::Message;
use bd2::proto::proto_net::{FishingInfoRequest, FishingInfoResponse, ItemDbInfo, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// First-ever call seeds the account's default map and grants the starter item bundle from
/// `FishingDefaultTable` (defaultMapId/defaultItemId/defaultItemCount/defaultItemType) — that
/// table's row IS real captured data, so this part is genuine, not a placeholder.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingInfoRequest) -> GameResponse {
    info!("Handling FishingInfoRequest: {:?}", req);

    let user = database::db::fishing::fishing_user_info::get_or_create(pool, uid)
        .await
        .unwrap_or_default();
    let mut map_ids = database::db::fishing::fishing_map_owned::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();
    let boat_skin_ids = database::db::fishing::fishing_boat_skin_owned::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();

    let mut reward_info_bundle = None;
    if map_ids.is_empty() {
        let default_row = data::exceldb::get().fishingdefaulttable.all().first().cloned();
        if let Some(default_row) = default_row {
            let _ = database::db::fishing::fishing_map_owned::add(pool, uid, default_row.default_map_id).await;
            map_ids.push(default_row.default_map_id);

            let mut granted = Vec::new();
            for i in 0..default_row.default_item_id.len() {
                let id = default_row.default_item_id[i];
                let r#type = default_row.default_item_type.get(i).copied().unwrap_or(0);
                let count = default_row.default_item_count.get(i).copied().unwrap_or(1);
                let _ = database::db::item::item_info::grant(pool, uid, id, r#type, count).await;
                granted.push(ItemDbInfo {
                    inven_index: None,
                    id: Some(id),
                    r#type: Some(r#type),
                    count: Some(count),
                    keep_flag: None,
                    time_value: None,
                    pictorialbook_info: None,
                    expiry_time: None,
                    sort_id: None,
                    use_count: None,
                });
            }
            reward_info_bundle = Some(RewardDbInfoBundle {
                item_info: granted,
                ..Default::default()
            });
        }
    }

    let response = FishingInfoResponse {
        fish_info: Some(user_to_dbinfo(&user)),
        map_id: map_ids,
        boat_skin_id: boat_skin_ids,
        reward_info_bundle,
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
    let (route, code) = PacketCodeType::FishingInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
