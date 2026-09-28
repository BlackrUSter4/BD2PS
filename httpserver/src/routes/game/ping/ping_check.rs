use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PingCheckRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::ping::ping_check;
use sqlx::SqlitePool;

#[put("PingCheck")]
async fn ping_check_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PingCheckRequest>("PingCheck", &body).map_err(|e| {
        tracing::warn!("Failed to parse PingCheck: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = ping_check::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
