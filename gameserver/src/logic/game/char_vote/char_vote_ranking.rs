use bd2::prost::Message;
use bd2::proto::proto_net::{CharVoteRankingDbInfo, CharVoteRankingRequest, CharVoteRankingResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, CURRENT_EVENT_ID, CURRENT_ROUND};

/// Ranking is a real cross-account aggregate (SUM of every account's saved votes for this round)
/// — not bot-padded, not fabricated, just real state that's currently sparse because there's
/// only whatever accounts have actually voted so far.
pub async fn handle(pool: &SqlitePool, _uid: i64, req: CharVoteRankingRequest) -> GameResponse {
    info!("Handling CharVoteRankingRequest: {:?}", req);

    let round = req.round.unwrap_or(CURRENT_ROUND);
    let totals = database::db::char_vote::char_vote_user_info::total_votes_by_candidate(pool, CURRENT_EVENT_ID, round)
        .await
        .unwrap_or_default();

    let ranking_info = totals
        .into_iter()
        .enumerate()
        .map(|(i, (candidate_id, vote_count))| CharVoteRankingDbInfo {
            candidate_id: Some(candidate_id),
            vote_count: Some(vote_count),
            rank: Some(i as i32 + 1),
            last_round: Some(round),
        })
        .collect();

    let response = CharVoteRankingResponse {
        event_id: Some(CURRENT_EVENT_ID),
        current_round: Some(CURRENT_ROUND),
        ranking_info,
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CharVoteRanking.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
