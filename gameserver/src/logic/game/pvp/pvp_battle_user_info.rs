use bd2::prost::Message;
use bd2::proto::proto_net::{PvpBattleUserBaseInfo, PvpBattleUserInfoRequest, PvpBattleUserInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pvp::pvp_user_info;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(pool: &SqlitePool, uid: i64, req: PvpBattleUserInfoRequest) -> GameResponse {
    info!("Handling PvpBattleUserInfoRequest: {:?}", req);

    let user = pvp_user_info::get_or_create(pool, uid).await.unwrap_or_default();
    let rank = pvp_user_info::rank_of(pool, uid).await.unwrap_or(1);
    let season = super::current_season();
    let is_season_reward = user.season_reward_claimed_season < season - 1 && season > 1;

    let response = PvpBattleUserInfoResponse {
        base_info: Some(PvpBattleUserBaseInfo {
            vp: Some(user.vp),
            rank: Some(rank),
            win_count: Some(user.win_count),
            lose_count: Some(user.lose_count),
        }),
        is_season_reward: Some(is_season_reward),
        pvp_table_change_season: Some(season),
        engine_type: None,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PvpBattleUserInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
