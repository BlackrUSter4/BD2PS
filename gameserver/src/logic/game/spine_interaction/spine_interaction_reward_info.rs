use bd2::prost::Message;
use bd2::proto::proto_net::{SpineInteractionRewardDbInfo, SpineInteractionRewardInfoRequest, SpineInteractionRewardInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::spine_interaction::spine_interaction_reward_claim;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(pool: &SqlitePool, uid: i64, req: SpineInteractionRewardInfoRequest) -> GameResponse {
    info!("Handling SpineInteractionRewardInfoRequest: {:?}", req);

    let rows = spine_interaction_reward_claim::list_for_uid(pool, uid)
        .await
        .unwrap_or_default();

    let spine_reward_info = rows
        .into_iter()
        .map(|r| SpineInteractionRewardDbInfo {
            interaction_group_id: Some(r.interaction_group_id),
            group_id: Some(r.group_id),
            id: Some(r.id),
        })
        .collect();

    let response = SpineInteractionRewardInfoResponse { spine_reward_info };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::SpineInteractionRewardInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
