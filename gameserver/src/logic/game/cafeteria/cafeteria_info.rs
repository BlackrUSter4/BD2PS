use bd2::prost::Message;
use bd2::proto::proto_net::{
    CafeteriaDbInfo, CafeteriaFacilityDbInfo, CafeteriaInfoRequest, CafeteriaInfoResponse,
    CafeteriaPartTimeManagerDbInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::cafeteria::{cafeteria_facility_info, cafeteria_info, cafeteria_part_time_manager_info};
use sqlx::SqlitePool;
use tracing::info;

use super::to_id_list;

pub async fn handle(pool: &SqlitePool, uid: i64, req: CafeteriaInfoRequest) -> GameResponse {
    info!("Handling CafeteriaInfoRequest: {:?}", req);

    let info_row = match cafeteria_info::get_or_create(pool, uid).await {
        Ok(row) => row,
        Err(e) => {
            tracing::error!("CafeteriaInfo get_or_create failed: {}", e);
            return GameResponse::error(1);
        }
    };

    let facilities = cafeteria_facility_info::get_cafeteria_facility_info(pool, uid)
        .await
        .unwrap_or_default();
    let managers = cafeteria_part_time_manager_info::get_cafeteria_part_time_manager_info(pool, uid)
        .await
        .unwrap_or_default();

    let response = CafeteriaInfoResponse {
        cafeteria_info: Some(CafeteriaDbInfo {
            level: info_row.level,
            reward_receipt_time: info_row.reward_receipt_time,
            spawn_time: info_row.spawn_time,
            ongoing_manage_id: info_row.ongoing_manage_id,
            daily_regular_costume_id: to_id_list(&info_row.daily_regular_costume_ids),
            rewarded_daily_regular_costume_id: to_id_list(&info_row.rewarded_daily_regular_costume_ids),
            daily_connection_costume_id: info_row.daily_connection_costume_id,
            can_get_phone_number: info_row.can_get_phone_number.map(|v| v != 0),
            daily_npc_reward_currency_count: info_row.daily_npc_reward_currency_count,
        }),
        facility_info: facilities
            .into_iter()
            .map(|f| CafeteriaFacilityDbInfo {
                facility_id: f.facility_id,
            })
            .collect(),
        part_time_manager_info: managers
            .into_iter()
            .map(|m| CafeteriaPartTimeManagerDbInfo {
                part_time_manager_id: m.part_time_manager_id,
            })
            .collect(),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CafeteriaInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
