use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameSurvivalStartRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_survival_start;
use sqlx::SqlitePool;

#[put("MiniGameSurvivalStart")]
async fn mini_game_survival_start_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MiniGameSurvivalStartRequest>("MiniGameSurvivalStart", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse MiniGameSurvivalStart: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = mini_game_survival_start::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
