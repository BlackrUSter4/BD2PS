use bd2::prost::Message;
use bd2::proto::proto_net::{
    CafeteriaDbInfo, CafeteriaLevelUpRequest, CafeteriaLevelUpResponse, ItemDbInfo,
    RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{cafeteria::cafeteria_info, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

use super::to_id_list;

/// No cost/reward table exists for Cafeteria level-up (`CafeteriaLevelTable` only carries
/// per-level unlock config) — leveling up is free, capped at the table's highest defined level,
/// and grants a flat placeholder gold amount.
pub async fn handle(pool: &SqlitePool, uid: i64, req: CafeteriaLevelUpRequest) -> GameResponse {
    info!("Handling CafeteriaLevelUpRequest: {:?}", req);

    let mut info_row = match cafeteria_info::get_or_create(pool, uid).await {
        Ok(row) => row,
        Err(e) => {
            tracing::error!("CafeteriaLevelUp get_or_create failed: {}", e);
            return GameResponse::error(1);
        }
    };

    let max_level = data::exceldb::get().cafeterialeveltable.all().len() as i32;
    let current = info_row.level.unwrap_or(1);
    let new_level = (current + 1).min(max_level.max(current));
    info_row.level = Some(new_level);

    if let Err(e) = cafeteria_info::update_cafeteria_info(pool, &info_row).await {
        tracing::error!("CafeteriaLevelUp update failed: {}", e);
        return GameResponse::error(1);
    }

    if new_level > current {
        let _ = item_info::grant(
            pool,
            uid,
            super::GOLD_ITEM_ID,
            super::GOLD_ITEM_TYPE,
            super::LEVEL_UP_REWARD_GOLD,
        )
        .await;
    }

    let response = CafeteriaLevelUpResponse {
        new_cafeteria_info: Some(CafeteriaDbInfo {
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
        cumulative_reward_info: Some(RewardDbInfoBundle {
            item_info: vec![ItemDbInfo {
                id: Some(super::GOLD_ITEM_ID),
                r#type: Some(super::GOLD_ITEM_TYPE),
                count: Some(super::LEVEL_UP_REWARD_GOLD),
                ..Default::default()
            }],
            ..Default::default()
        }),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CafeteriaLevelUp.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
