use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameSurvivalEndRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_survival_end;
use sqlx::SqlitePool;

#[put("MiniGameSurvivalEnd")]
async fn mini_game_survival_end_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<MiniGameSurvivalEndRequest>("MiniGameSurvivalEnd", &body).map_err(|e| {
            tracing::warn!("Failed to parse MiniGameSurvivalEnd: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = mini_game_survival_end::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
