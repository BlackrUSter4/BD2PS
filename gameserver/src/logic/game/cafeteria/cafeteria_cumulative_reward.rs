use bd2::prost::Message;
use bd2::proto::proto_net::{
    CafeteriaCumulativeRewardRequest, CafeteriaCumulativeRewardResponse, ItemDbInfo,
    RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{cafeteria::cafeteria_info, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

/// Passive idle-income claim. `CafeteriaDefaultTable.min_reward_time`/`max_reward_time` (real,
/// in minutes) bound how much elapsed time can accrue, but no table anywhere states an actual
/// gold-per-minute rate — `CUMULATIVE_REWARD_GOLD_PER_MINUTE` is a placeholder. Timing/capping
/// itself is real: computed from real elapsed wall-clock time since the account's own last claim.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: CafeteriaCumulativeRewardRequest,
) -> GameResponse {
    info!("Handling CafeteriaCumulativeRewardRequest: {:?}", req);

    let mut info_row = match cafeteria_info::get_or_create(pool, uid).await {
        Ok(row) => row,
        Err(e) => {
            tracing::error!("CafeteriaCumulativeReward get_or_create failed: {}", e);
            return GameResponse::error(1);
        }
    };

    let default = data::exceldb::get().cafeteriadefaulttable.all().first().cloned();
    let (min_minutes, max_minutes) = default
        .map(|d| (d.min_reward_time, d.max_reward_time))
        .unwrap_or((10, 1440));

    let now_ms = chrono::Utc::now().timestamp_millis();
    let last = info_row.reward_receipt_time.unwrap_or(now_ms);
    let elapsed_minutes = ((now_ms - last).max(0) / 60_000) as i32;
    let claimable_minutes = if elapsed_minutes >= min_minutes {
        elapsed_minutes.min(max_minutes)
    } else {
        0
    };

    let gold = claimable_minutes * super::CUMULATIVE_REWARD_GOLD_PER_MINUTE;

    let mut item_infos = Vec::new();
    if gold > 0 {
        let _ = item_info::grant(pool, uid, super::GOLD_ITEM_ID, super::GOLD_ITEM_TYPE, gold).await;
        item_infos.push(ItemDbInfo {
            id: Some(super::GOLD_ITEM_ID),
            r#type: Some(super::GOLD_ITEM_TYPE),
            count: Some(gold),
            ..Default::default()
        });
    }

    info_row.reward_receipt_time = Some(now_ms);
    if let Err(e) = cafeteria_info::update_cafeteria_info(pool, &info_row).await {
        tracing::error!("CafeteriaCumulativeReward update failed: {}", e);
    }

    let response = CafeteriaCumulativeRewardResponse {
        cumulative_reward_info: Some(RewardDbInfoBundle {
            item_info: item_infos,
            ..Default::default()
        }),
        reward_receipt_time: info_row.reward_receipt_time,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CafeteriaCumulativeReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
