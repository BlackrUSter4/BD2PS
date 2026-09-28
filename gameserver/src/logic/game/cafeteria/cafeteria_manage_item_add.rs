use bd2::prost::Message;
use bd2::proto::proto_net::{
    CafeteriaFacilityDbInfo, CafeteriaManageItemAddRequest, CafeteriaManageItemAddResponse,
    CafeteriaPartTimeManagerDbInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::cafeteria::{cafeteria_facility_info, cafeteria_info, cafeteria_part_time_manager_info};
use database::db::item::item_info;
use database::models::game::cafeteria::{
    cafeteria_facility_info::CafeteriaFacilityInfo,
    cafeteria_part_time_manager_info::CafeteriaPartTimeManagerInfo,
};
use sqlx::SqlitePool;
use tracing::info;

/// `CafeteriaManageTable`'s 121 rows have no explicit unlock-order field (their (groupId, id)
/// pair resets per group) — the request itself carries no target id either
/// (`CafeteriaManageItemAddRequest{ seq }` only), so the server must be deciding which catalog
/// entry comes next. Implemented as: walk the captured table's own row order one at a time via
/// `CafeteriaInfo.ongoing_manage_id` as a plain 0-based index, charging that row's real
/// `costValue` (in gold — `costType` is a constant 40 across every row with nothing else to map
/// it to) via real `item_info::consume`. If the account can't afford it (or the catalog is
/// exhausted), nothing changes and no item is added.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: CafeteriaManageItemAddRequest,
) -> GameResponse {
    info!("Handling CafeteriaManageItemAddRequest: {:?}", req);

    let mut info_row = match cafeteria_info::get_or_create(pool, uid).await {
        Ok(row) => row,
        Err(e) => {
            tracing::error!("CafeteriaManageItemAdd get_or_create failed: {}", e);
            return GameResponse::error(1);
        }
    };

    let idx = info_row.ongoing_manage_id.unwrap_or(0).max(0) as usize;
    let table = data::exceldb::get().cafeteriamanagetable.all();

    let mut added_facility_info = None;
    let mut added_part_time_manager_info = None;

    if let Some(row) = table.get(idx) {
        let cost = row.cost_value.unwrap_or(0);
        let can_afford = if cost <= 0 {
            true
        } else {
            item_info::consume(pool, uid, super::GOLD_ITEM_ID, cost)
                .await
                .unwrap_or(false)
        };

        if can_afford {
            if let Some(facility_id) = row.facility_id {
                let _ = cafeteria_facility_info::insert(
                    pool,
                    &CafeteriaFacilityInfo {
                        index: 0,
                        uid,
                        facility_id: Some(facility_id),
                    },
                )
                .await;
                added_facility_info = Some(CafeteriaFacilityDbInfo {
                    facility_id: Some(facility_id),
                });
            }
            if let Some(parttime_id) = row.parttime_id {
                let _ = cafeteria_part_time_manager_info::insert(
                    pool,
                    &CafeteriaPartTimeManagerInfo {
                        index: 0,
                        uid,
                        part_time_manager_id: Some(parttime_id),
                    },
                )
                .await;
                added_part_time_manager_info = Some(CafeteriaPartTimeManagerDbInfo {
                    part_time_manager_id: Some(parttime_id),
                });
            }

            info_row.ongoing_manage_id = Some((idx + 1) as i32);
            if let Err(e) = cafeteria_info::update_cafeteria_info(pool, &info_row).await {
                tracing::error!("CafeteriaManageItemAdd update failed: {}", e);
            }
        }
    }

    let response = CafeteriaManageItemAddResponse {
        next_manage_id: info_row.ongoing_manage_id,
        added_facility_info,
        added_part_time_manager_info,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CafeteriaManageItemAdd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
