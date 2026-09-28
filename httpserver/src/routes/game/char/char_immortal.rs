use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharImmortalRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char::char_immortal;
use sqlx::SqlitePool;

#[put("CharImmortal")]
async fn char_immortal_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CharImmortalRequest>("CharImmortal", &body).map_err(|e| {
        tracing::warn!("Failed to parse CharImmortal: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = char_immortal::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
