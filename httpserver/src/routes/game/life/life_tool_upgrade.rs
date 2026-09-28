use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LifeToolUpgradeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::life::life_tool_upgrade;
use sqlx::SqlitePool;

#[put("LifeToolUpgrade")]
async fn life_tool_upgrade_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LifeToolUpgradeRequest>("LifeToolUpgrade", &body).map_err(|e| {
        tracing::warn!("Failed to parse LifeToolUpgrade: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = life_tool_upgrade::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
