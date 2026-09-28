use bd2::prost::Message;
use bd2::proto::proto_net::{DailyStoryClearRequest, DailyStoryClearResponse, ItemDbInfo, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{daily_story::daily_story_clear_info, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

use super::{DAILY_STORY_REWARD_COUNT, DAILY_STORY_REWARD_ITEM_ID, DAILY_STORY_REWARD_ITEM_TYPE};

pub async fn handle(pool: &SqlitePool, uid: i64, req: DailyStoryClearRequest) -> GameResponse {
    info!("Handling DailyStoryClearRequest: {:?}", req);

    let id = req.id.unwrap_or_default();
    let newly_cleared = daily_story_clear_info::try_clear(pool, uid, id).await;

    let reward_info_bundle = if newly_cleared {
        let _ = item_info::grant(pool, uid, DAILY_STORY_REWARD_ITEM_ID, DAILY_STORY_REWARD_ITEM_TYPE, DAILY_STORY_REWARD_COUNT).await;
        RewardDbInfoBundle {
            item_info: vec![ItemDbInfo {
                id: Some(DAILY_STORY_REWARD_ITEM_ID),
                r#type: Some(DAILY_STORY_REWARD_ITEM_TYPE),
                count: Some(DAILY_STORY_REWARD_COUNT),
                ..Default::default()
            }],
            ..Default::default()
        }
    } else {
        RewardDbInfoBundle::default()
    };

    let response = DailyStoryClearResponse {
        reward_info_bundle: Some(reward_info_bundle),
        clear_info: Some(id),
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::DailyStoryClear.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
