use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameSichuanRankingRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_sichuan_ranking;
use sqlx::SqlitePool;

#[put("MiniGameSichuanRanking")]
async fn mini_game_sichuan_ranking_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MiniGameSichuanRankingRequest>("MiniGameSichuanRanking", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse MiniGameSichuanRanking: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = mini_game_sichuan_ranking::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
