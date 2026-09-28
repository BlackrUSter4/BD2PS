use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EvilCastlePingRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::evil::evil_castle_ping;
use sqlx::SqlitePool;

#[put("EvilCastlePing")]
async fn evil_castle_ping_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EvilCastlePingRequest>("EvilCastlePing", &body).map_err(|e| {
        tracing::warn!("Failed to parse EvilCastlePing: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = evil_castle_ping::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
