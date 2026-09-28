use bd2::prost::Message;
use bd2::proto::proto_net::{ColosseumPromotionRewardInfoRequest, ColosseumPromotionRewardInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::colosseum::colosseum_promotion_reward;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: ColosseumPromotionRewardInfoRequest,
) -> GameResponse {
    info!("Handling ColosseumPromotionRewardInfoRequest: {:?}", req);

    let rows = colosseum_promotion_reward::get_by_uid(pool, uid).await.unwrap_or_default();
    let promotion_reward_id = rows.into_iter().map(|r| r.reward_id).collect();

    let response = ColosseumPromotionRewardInfoResponse { promotion_reward_id };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumPromotionRewardInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
