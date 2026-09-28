use bd2::prost::Message;
use bd2::proto::proto_net::{
    CharVoteSeasonRankingDbInfo, CharVoteSeasonRankingRequest, CharVoteSeasonRankingResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, CURRENT_EVENT_ID};

/// "Season" here == the current event's all-round total (no separate season/event distinction
/// exists anywhere in the captured/decompiled data) — real cross-account aggregate.
pub async fn handle(pool: &SqlitePool, _uid: i64, req: CharVoteSeasonRankingRequest) -> GameResponse {
    info!("Handling CharVoteSeasonRankingRequest: {:?}", req);

    let totals = database::db::char_vote::char_vote_user_info::total_votes_across_rounds(pool, CURRENT_EVENT_ID)
        .await
        .unwrap_or_default();

    let season_ranking_info = totals
        .into_iter()
        .map(|(candidate_id, total)| CharVoteSeasonRankingDbInfo {
            event_id: Some(CURRENT_EVENT_ID),
            candidate_id: Some(candidate_id),
            total_vote_count: Some(total),
            vote_count: Some(total),
        })
        .collect();

    let response = CharVoteSeasonRankingResponse { season_ranking_info };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CharVoteSeasonRanking.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
