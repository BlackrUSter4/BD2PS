use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ColosseumRankingRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::colosseum::colosseum_ranking;
use sqlx::SqlitePool;

#[put("ColosseumRanking")]
async fn colosseum_ranking_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ColosseumRankingRequest>("ColosseumRanking", &body).map_err(|e| {
        tracing::warn!("Failed to parse ColosseumRanking: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = colosseum_ranking::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
