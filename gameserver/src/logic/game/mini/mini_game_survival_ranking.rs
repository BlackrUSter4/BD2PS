use bd2::prost::Message;
use bd2::proto::proto_net::{
    MiniGameSurvivalRankDbInfo, MiniGameSurvivalRankingRequest, MiniGameSurvivalRankingResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_survival_rank_info as rank_db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

const RANKING_LIMIT: i64 = 100;

/// Real cross-account leaderboard, same pattern as Action/Rhythm/Sichuan.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameSurvivalRankingRequest) -> GameResponse {
    info!("Handling MiniGameSurvivalRankingRequest: {:?}", req);

    let top = rank_db::get_top(pool, RANKING_LIMIT).await.unwrap_or_default();
    let rank_info: Vec<MiniGameSurvivalRankDbInfo> = top
        .iter()
        .enumerate()
        .map(|(i, r)| MiniGameSurvivalRankDbInfo {
            rank: Some(i as i32 + 1),
            owner_index: r.owner_index,
            user_id: r.user_id.clone(),
            point: r.point,
        })
        .collect();

    let user_rank_info = rank_db::get_own(pool, uid).await.ok().flatten().map(|r| {
        let rank = top.iter().position(|t| t.uid == r.uid).map(|p| p as i32 + 1);
        MiniGameSurvivalRankDbInfo { rank, owner_index: r.owner_index, user_id: r.user_id, point: r.point }
    });

    let response = MiniGameSurvivalRankingResponse { rank_info, user_rank_info };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameSurvivalRanking.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
