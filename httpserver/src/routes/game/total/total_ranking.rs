use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::TotalRankingRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::total::total_ranking;
use sqlx::SqlitePool;

#[put("TotalRanking")]
async fn total_ranking_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<TotalRankingRequest>("TotalRanking", &body).map_err(|e| {
        tracing::warn!("Failed to parse TotalRanking: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = total_ranking::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
