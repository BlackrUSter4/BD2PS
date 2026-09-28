use bd2::prost::Message;
use bd2::proto::proto_net::{ColosseumUserInfoRequest, ColosseumUserInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::colosseum::{colosseum_season_reward, colosseum_user_info};
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, user_base_info, CURRENT_SEASON, RANK_TABLE_CHANGE_SEASON, REGULAR_SEASON};

pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumUserInfoRequest) -> GameResponse {
    info!("Handling ColosseumUserInfoRequest: {:?}", req);

    let user = colosseum_user_info::get_or_create(pool, uid).await.unwrap_or_default();
    let rank = colosseum_user_info::rank_of(pool, uid).await.unwrap_or(1);
    let prev_season_unclaimed = if user.season > 1 {
        !colosseum_season_reward::is_claimed(pool, uid, user.season - 1)
            .await
            .unwrap_or(true)
    } else {
        false
    };

    let response = ColosseumUserInfoResponse {
        season: Some(CURRENT_SEASON),
        regular_season: Some(REGULAR_SEASON),
        rank_table_change_season: Some(RANK_TABLE_CHANGE_SEASON),
        base_info: Some(user_base_info(&user, rank)),
        is_season_reward: Some(prev_season_unclaimed),
        ap_buy_count: Some(user.ap_buy_count),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumUserInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
