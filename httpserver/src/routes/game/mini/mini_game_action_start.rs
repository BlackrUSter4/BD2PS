use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameActionStartRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_action_start;
use sqlx::SqlitePool;

#[put("MiniGameActionStart")]
async fn mini_game_action_start_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<MiniGameActionStartRequest>("MiniGameActionStart", &body).map_err(|e| {
            tracing::warn!("Failed to parse MiniGameActionStart: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = mini_game_action_start::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
