use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MaintenanceInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::maintenance::maintenance_info;
use sqlx::SqlitePool;

#[put("MaintenanceInfo")]
async fn maintenance_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MaintenanceInfoRequest>("MaintenanceInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse MaintenanceInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = maintenance_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
