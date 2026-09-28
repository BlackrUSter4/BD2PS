use bd2::prost::Message;
use bd2::proto::proto_net::{FriendshipSpecialEpisodeClearRequest, FriendshipSpecialEpisodeClearResponse, FriendshipSpecialEpisodeDbInfo, ItemDbInfo, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::friendship::friendship_special_episode;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

use super::{SPECIAL_EPISODE_REWARD_COUNT, SPECIAL_EPISODE_REWARD_ITEM_ID, SPECIAL_EPISODE_REWARD_ITEM_TYPE};

pub async fn handle(pool: &SqlitePool, uid: i64, req: FriendshipSpecialEpisodeClearRequest) -> GameResponse {
    info!("Handling FriendshipSpecialEpisodeClearRequest: {:?}", req);

    let group_id = req.group_id.unwrap_or_default();
    let id = req.id.unwrap_or_default();

    let newly_cleared = friendship_special_episode::try_clear(pool, uid, group_id, id).await;

    let reward_info_bundle = if newly_cleared {
        let _ = item_info::grant(pool, uid, SPECIAL_EPISODE_REWARD_ITEM_ID, SPECIAL_EPISODE_REWARD_ITEM_TYPE, SPECIAL_EPISODE_REWARD_COUNT).await;
        RewardDbInfoBundle {
            item_info: vec![ItemDbInfo {
                id: Some(SPECIAL_EPISODE_REWARD_ITEM_ID),
                r#type: Some(SPECIAL_EPISODE_REWARD_ITEM_TYPE),
                count: Some(SPECIAL_EPISODE_REWARD_COUNT),
                ..Default::default()
            }],
            ..Default::default()
        }
    } else {
        RewardDbInfoBundle::default()
    };

    let response = FriendshipSpecialEpisodeClearResponse {
        reward_info_bundle: Some(reward_info_bundle),
        clear_info: Some(FriendshipSpecialEpisodeDbInfo {
            group_id: Some(group_id),
            id: Some(id),
        }),
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::FriendshipSpecialEpisodeClear.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
