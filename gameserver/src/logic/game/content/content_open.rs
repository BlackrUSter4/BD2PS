use bd2::prost::Message;
use bd2::proto::proto_net::{ContentOpenRequest, ContentOpenResponse, ItemDbInfo, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{content::content_open_info, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

use super::{CONTENT_OPEN_REWARD_COUNT, CONTENT_OPEN_REWARD_ITEM_ID, CONTENT_OPEN_REWARD_ITEM_TYPE};

/// Real claim-once tracking per (uid, type); reward contents are a placeholder (see
/// `content::CONTENT_OPEN_REWARD_*`) since no table maps a content type to a real reward.
pub async fn handle(pool: &SqlitePool, uid: i64, req: ContentOpenRequest) -> GameResponse {
    info!("Handling ContentOpenRequest: {:?}", req);

    let r#type = req.r#type.unwrap_or_default();
    let newly_opened = content_open_info::try_open(pool, uid, r#type).await;

    let reward_info_bundle = if newly_opened {
        let _ = item_info::grant(pool, uid, CONTENT_OPEN_REWARD_ITEM_ID, CONTENT_OPEN_REWARD_ITEM_TYPE, CONTENT_OPEN_REWARD_COUNT).await;
        RewardDbInfoBundle {
            item_info: vec![ItemDbInfo {
                id: Some(CONTENT_OPEN_REWARD_ITEM_ID),
                r#type: Some(CONTENT_OPEN_REWARD_ITEM_TYPE),
                count: Some(CONTENT_OPEN_REWARD_COUNT),
                ..Default::default()
            }],
            ..Default::default()
        }
    } else {
        RewardDbInfoBundle::default()
    };

    let response = ContentOpenResponse {
        reward_info_bundle: Some(reward_info_bundle),
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ContentOpen.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
