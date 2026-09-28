use bd2::prost::Message;
use bd2::proto::proto_net::{ColosseumRankUserInfo, ColosseumRankingRequest, ColosseumRankingResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::colosseum::colosseum_user_info;
use database::models::game::colosseum::colosseum_user_info::ColosseumUserInfo;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, user_base_info};

async fn to_rank_user_info(pool: &SqlitePool, row: &ColosseumUserInfo, rank: i32) -> ColosseumRankUserInfo {
    let account = database::db::user::user::find_account(pool, row.uid).await.ok().flatten();
    ColosseumRankUserInfo {
        owner_index: Some(row.uid),
        user_id: account.map(|a| a.user_name),
        user_exp: Some(0),
        portrait_costume_id: None,
        portrait_costume_design_id: None,
        guild_base_info: None,
        base_info: Some(user_base_info(row, rank)),
        title_id: None,
    }
}

/// Ranking is computed only over real accounts in this server's own database — there's no
/// bot-filled leaderboard here (unlike matching, where a thin bot fallback keeps the battle
/// loop playable solo; a padded-looking leaderboard would be actively misleading instead).
pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumRankingRequest) -> GameResponse {
    info!("Handling ColosseumRankingRequest: {:?}", req);

    let top = colosseum_user_info::top_by_vp(pool, 100).await.unwrap_or_default();
    let mut user_ranking_info = Vec::with_capacity(top.len());
    for (i, row) in top.iter().enumerate() {
        user_ranking_info.push(to_rank_user_info(pool, row, i as i32 + 1).await);
    }

    let my_row = colosseum_user_info::get_or_create(pool, uid).await.unwrap_or_default();
    let my_rank = colosseum_user_info::rank_of(pool, uid).await.unwrap_or(1);
    let my_ranking_info = Some(to_rank_user_info(pool, &my_row, my_rank).await);

    let response = ColosseumRankingResponse { user_ranking_info, my_ranking_info };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumRanking.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
