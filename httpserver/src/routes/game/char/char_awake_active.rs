use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharAwakeActiveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char::char_awake_active;
use sqlx::SqlitePool;

#[put("CharAwakeActive")]
async fn char_awake_active_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CharAwakeActiveRequest>("CharAwakeActive", &body).map_err(|e| {
        tracing::warn!("Failed to parse CharAwakeActive: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = char_awake_active::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
