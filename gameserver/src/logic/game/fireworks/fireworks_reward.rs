use bd2::prost::Message;
use bd2::proto::proto_net::{FireWorksRewardRequest, FireWorksRewardResponse, ItemDbInfo, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{fireworks::fireworks_reward_info, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

use super::{FIREWORKS_REWARD_COUNT, FIREWORKS_REWARD_ITEM_ID, FIREWORKS_REWARD_ITEM_TYPE};

pub async fn handle(pool: &SqlitePool, uid: i64, req: FireWorksRewardRequest) -> GameResponse {
    info!("Handling FireWorksRewardRequest: {:?}", req);

    let event_schedule_id = req.event_schedule_id.unwrap_or_default();
    let group_id = req.group_id.unwrap_or_default();
    let newly_claimed = fireworks_reward_info::try_claim(pool, uid, event_schedule_id, group_id).await;

    let reward_bundle = if newly_claimed {
        let _ = item_info::grant(pool, uid, FIREWORKS_REWARD_ITEM_ID, FIREWORKS_REWARD_ITEM_TYPE, FIREWORKS_REWARD_COUNT).await;
        RewardDbInfoBundle {
            item_info: vec![ItemDbInfo {
                id: Some(FIREWORKS_REWARD_ITEM_ID),
                r#type: Some(FIREWORKS_REWARD_ITEM_TYPE),
                count: Some(FIREWORKS_REWARD_COUNT),
                ..Default::default()
            }],
            ..Default::default()
        }
    } else {
        RewardDbInfoBundle::default()
    };

    let response = FireWorksRewardResponse {
        reward_bundle: Some(reward_bundle),
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::FireWorksReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
