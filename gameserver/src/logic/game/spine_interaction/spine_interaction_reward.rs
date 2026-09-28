use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, RewardDbInfoBundle, SpineInteractionRewardRequest, SpineInteractionRewardResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, spine_interaction::spine_interaction_reward_claim};
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, PLACEHOLDER_REWARD_COUNT, PLACEHOLDER_REWARD_ITEM_ID, PLACEHOLDER_REWARD_ITEM_TYPE};

/// See mod.rs doc comment on the placeholder constants: no reward-table data links a
/// (interaction_group_id, group_id, id) triple to a real reward anywhere in the captured master
/// data, so this grants a fixed placeholder amount rather than fabricating a lookup. The claim
/// itself (preventing a double-grant on repeat calls) is real.
pub async fn handle(pool: &SqlitePool, uid: i64, req: SpineInteractionRewardRequest) -> GameResponse {
    info!("Handling SpineInteractionRewardRequest: {:?}", req);

    let (interaction_group_id, group_id, id) = req
        .spine_reward_info
        .as_ref()
        .map(|i| (i.interaction_group_id.unwrap_or_default(), i.group_id.unwrap_or_default(), i.id.unwrap_or_default()))
        .unwrap_or_default();

    let newly_claimed = spine_interaction_reward_claim::claim(pool, uid, interaction_group_id, group_id, id)
        .await
        .unwrap_or(false);

    let reward_info_bundle = if newly_claimed {
        let _ = item_info::grant(pool, uid, PLACEHOLDER_REWARD_ITEM_ID, PLACEHOLDER_REWARD_ITEM_TYPE, PLACEHOLDER_REWARD_COUNT).await;
        Some(RewardDbInfoBundle {
            item_info: vec![ItemDbInfo {
                id: Some(PLACEHOLDER_REWARD_ITEM_ID),
                r#type: Some(PLACEHOLDER_REWARD_ITEM_TYPE),
                count: Some(PLACEHOLDER_REWARD_COUNT),
                ..Default::default()
            }],
            ..Default::default()
        })
    } else {
        // Already claimed — no re-grant, honest empty bundle.
        Some(RewardDbInfoBundle::default())
    };

    let response = SpineInteractionRewardResponse { reward_info_bundle };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::SpineInteractionReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
