use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ClientCustomLogRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::client::client_custom_log;
use sqlx::SqlitePool;

#[put("ClientCustomLog")]
async fn client_custom_log_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ClientCustomLogRequest>("ClientCustomLog", &body).map_err(|e| {
        tracing::warn!("Failed to parse ClientCustomLog: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = client_custom_log::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
