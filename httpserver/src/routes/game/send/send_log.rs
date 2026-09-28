use actix_web::{HttpResponse, Result, put, web};
use bd2::SendLogRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::send::send_log;
use sqlx::SqlitePool;

#[put("SendLog")]
async fn send_log_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<SendLogRequest>("SendLog", &body).map_err(|e| {
        tracing::warn!("Failed to parse SendLog: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = send_log::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
