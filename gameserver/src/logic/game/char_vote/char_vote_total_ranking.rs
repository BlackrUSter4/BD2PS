use bd2::prost::Message;
use bd2::proto::proto_net::{CharVoteRankingDbInfo, CharVoteTotalRankingRequest, CharVoteTotalRankingResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, CURRENT_EVENT_ID};

/// Unlike CharVoteRanking (scoped to one round), the client supplies its own `event_id` here —
/// real cross-account, all-rounds aggregate for whichever event it asks about.
pub async fn handle(pool: &SqlitePool, _uid: i64, req: CharVoteTotalRankingRequest) -> GameResponse {
    info!("Handling CharVoteTotalRankingRequest: {:?}", req);

    let event_id = req.event_id.unwrap_or(CURRENT_EVENT_ID);
    let totals = database::db::char_vote::char_vote_user_info::total_votes_across_rounds(pool, event_id)
        .await
        .unwrap_or_default();

    let ranking_info = totals
        .into_iter()
        .enumerate()
        .map(|(i, (candidate_id, vote_count))| CharVoteRankingDbInfo {
            candidate_id: Some(candidate_id),
            vote_count: Some(vote_count),
            rank: Some(i as i32 + 1),
            last_round: None,
        })
        .collect();

    let response = CharVoteTotalRankingResponse { event_id: Some(event_id), ranking_info };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CharVoteTotalRanking.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
