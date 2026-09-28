use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameSurvivalPlayRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_survival_play;
use sqlx::SqlitePool;

#[put("MiniGameSurvivalPlay")]
async fn mini_game_survival_play_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MiniGameSurvivalPlayRequest>("MiniGameSurvivalPlay", &body).map_err(
        |e| {
            tracing::warn!("Failed to parse MiniGameSurvivalPlay: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        },
    )?;
    let response = mini_game_survival_play::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
