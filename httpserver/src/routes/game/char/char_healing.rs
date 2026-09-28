use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharHealingRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char::char_healing;
use sqlx::SqlitePool;

#[put("CharHealing")]
async fn char_healing_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CharHealingRequest>("CharHealing", &body).map_err(|e| {
        tracing::warn!("Failed to parse CharHealing: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = char_healing::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
