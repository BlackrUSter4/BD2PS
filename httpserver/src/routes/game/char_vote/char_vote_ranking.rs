use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharVoteRankingRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char_vote::char_vote_ranking;
use sqlx::SqlitePool;

#[put("CharVoteRanking")]
async fn char_vote_ranking_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CharVoteRankingRequest>("CharVoteRanking", &body).map_err(|e| {
        tracing::warn!("Failed to parse CharVoteRanking: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = char_vote_ranking::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
