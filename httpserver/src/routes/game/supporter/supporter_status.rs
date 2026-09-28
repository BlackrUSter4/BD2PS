use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SupporterStatusRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::supporter::supporter_status;
use sqlx::SqlitePool;

#[put("SupporterStatus")]
async fn supporter_status_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<SupporterStatusRequest>("SupporterStatus", &body).map_err(|e| {
        tracing::warn!("Failed to parse SupporterStatus: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = supporter_status::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
