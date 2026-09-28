use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharVoteTotalRankingRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char_vote::char_vote_total_ranking;
use sqlx::SqlitePool;

#[put("CharVoteTotalRanking")]
async fn char_vote_total_ranking_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CharVoteTotalRankingRequest>("CharVoteTotalRanking", &body).map_err(|e| {
        tracing::warn!("Failed to parse CharVoteTotalRanking: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = char_vote_total_ranking::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
