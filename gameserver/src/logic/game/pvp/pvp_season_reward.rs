use bd2::prost::Message;
use bd2::proto::proto_net::{PvpSeasonRewardRequest, PvpSeasonRewardResponse, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pvp::{pvp_season_reward, pvp_user_info};
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, grant_season_reward, now_ms, parse_claimed};

pub async fn handle(pool: &SqlitePool, uid: i64, req: PvpSeasonRewardRequest) -> GameResponse {
    info!("Handling PvpSeasonRewardRequest: {:?}", req);

    let user = pvp_user_info::get_or_create(pool, uid).await.unwrap_or_default();
    let rank = pvp_user_info::rank_of(pool, uid).await.unwrap_or(1);
    let claim_season = (super::current_season() - 1).max(1);

    let item_info = if !pvp_season_reward::is_claimed(pool, uid, claim_season).await.unwrap_or(false) {
        let items = grant_season_reward(pool, uid, user.vp).await;
        let _ = pvp_season_reward::claim(pool, uid, claim_season, now_ms()).await;
        let _ = pvp_user_info::set_season_reward_claimed(pool, uid, claim_season).await;
        items
    } else {
        vec![]
    };

    let response = PvpSeasonRewardResponse {
        rank: Some(rank),
        vp: Some(user.vp),
        reward_info_bundle: Some(RewardDbInfoBundle { item_info, ..Default::default() }),
        reward_season: Some(claim_season),
        once_reward_info: parse_claimed(&user.once_reward_claimed),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PvpSeasonReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
