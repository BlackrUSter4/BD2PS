use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameActionRankingRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_action_ranking;
use sqlx::SqlitePool;

#[put("MiniGameActionRanking")]
async fn mini_game_action_ranking_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MiniGameActionRankingRequest>("MiniGameActionRanking", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse MiniGameActionRanking: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = mini_game_action_ranking::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
