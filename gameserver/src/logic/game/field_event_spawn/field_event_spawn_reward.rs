use bd2::prost::Message;
use bd2::proto::proto_net::{FieldEventSpawnRewardRequest, FieldEventSpawnRewardResponse, ItemDbInfo, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{
    field_event_spawn::{field_event_spawn_daily_count, field_event_spawn_progress_info as progress_db},
    item::item_info,
};
use sqlx::SqlitePool;
use tracing::info;

use super::{CaughtEntry, FIELD_EVENT_SPAWN_REWARD_COUNT, FIELD_EVENT_SPAWN_REWARD_ITEM_ID, FIELD_EVENT_SPAWN_REWARD_ITEM_TYPE};

/// Real: records the catch in the session's progress row and bumps the real daily-count
/// column, then grants a placeholder reward (see module doc for why). Nothing in this
/// request distinguishes a "normal" vs "special" catch, so every catch counts toward
/// `daily_normal_count` — flagged as a judgment call, revisit if that distinction ever
/// surfaces in captured data.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FieldEventSpawnRewardRequest) -> GameResponse {
    info!("Handling FieldEventSpawnRewardRequest: {:?}", req);

    if let Some(mut row) = progress_db::get(pool, uid).await {
        let mut caught = super::parse_caught(&row.caught_info);
        caught.push(CaughtEntry {
            spawn_event_id: req.spawn_event_id.unwrap_or_default(),
            monster_group_id: req.monster_group_id.unwrap_or_default(),
            monster_id: req.monster_id.unwrap_or_default(),
        });
        row.caught_info = super::encode_caught(&caught);
        let _ = progress_db::save(pool, &row).await;
    }

    let today = chrono::Utc::now().date_naive().to_string();
    let _ = field_event_spawn_daily_count::increment(pool, uid, &today, false).await;

    let _ = item_info::grant(pool, uid, FIELD_EVENT_SPAWN_REWARD_ITEM_ID, FIELD_EVENT_SPAWN_REWARD_ITEM_TYPE, FIELD_EVENT_SPAWN_REWARD_COUNT).await;
    let reward_bundle = RewardDbInfoBundle {
        item_info: vec![ItemDbInfo {
            id: Some(FIELD_EVENT_SPAWN_REWARD_ITEM_ID),
            r#type: Some(FIELD_EVENT_SPAWN_REWARD_ITEM_TYPE),
            count: Some(FIELD_EVENT_SPAWN_REWARD_COUNT),
            ..Default::default()
        }],
        ..Default::default()
    };

    let response = FieldEventSpawnRewardResponse {
        reward_bundle: Some(reward_bundle),
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::FieldEventSpawnReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
