use bd2::prost::Message;
use bd2::proto::proto_net::{ColosseumSeasonRewardRequest, ColosseumSeasonRewardResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::colosseum::{colosseum_season_reward, colosseum_user_info};
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// No `ColosseumRankTable`/season-reward table data was captured, so the actual reward
/// contents can't be sourced — this genuinely marks the season as claimed (real state) but
/// the reward bundle itself is honestly empty rather than fabricated.
pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumSeasonRewardRequest) -> GameResponse {
    info!("Handling ColosseumSeasonRewardRequest: {:?}", req);

    let user = colosseum_user_info::get_or_create(pool, uid).await.unwrap_or_default();
    let claim_season = (user.season - 1).max(1);
    let _ = colosseum_season_reward::claim(pool, uid, claim_season).await;
    let rank = colosseum_user_info::rank_of(pool, uid).await.unwrap_or(1);
    let total = (user.win_count + user.lose_count).max(1);

    let response = ColosseumSeasonRewardResponse {
        top_percent: Some(100.0 * rank as f64 / total.max(rank) as f64),
        vp: Some(user.vp),
        reward_info_bundle: Some(super::empty_reward_bundle()),
        reward_season: Some(claim_season),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumSeasonReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
