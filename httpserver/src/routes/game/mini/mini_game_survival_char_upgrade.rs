use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameSurvivalCharUpgradeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_survival_char_upgrade;
use sqlx::SqlitePool;

#[put("MiniGameSurvivalCharUpgrade")]
async fn mini_game_survival_char_upgrade_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<MiniGameSurvivalCharUpgradeRequest>("MiniGameSurvivalCharUpgrade", &body)
            .map_err(|e| {
                tracing::warn!("Failed to parse MiniGameSurvivalCharUpgrade: {}", e);
                actix_web::error::ErrorBadRequest("Invalid packet")
            })?;
    let response = mini_game_survival_char_upgrade::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
