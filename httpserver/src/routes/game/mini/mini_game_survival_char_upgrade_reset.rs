use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameSurvivalCharUpgradeResetRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_survival_char_upgrade_reset;
use sqlx::SqlitePool;

#[put("MiniGameSurvivalCharUpgradeReset")]
async fn mini_game_survival_char_upgrade_reset_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MiniGameSurvivalCharUpgradeResetRequest>(
        "MiniGameSurvivalCharUpgradeReset",
        &body,
    )
    .map_err(|e| {
        tracing::warn!("Failed to parse MiniGameSurvivalCharUpgradeReset: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = mini_game_survival_char_upgrade_reset::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
