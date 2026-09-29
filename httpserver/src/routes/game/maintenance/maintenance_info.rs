use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MaintenanceInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::maintenance::maintenance_info;
use sqlx::SqlitePool;

#[put("MaintenanceInfo")]
async fn maintenance_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
) -> Result<HttpResponse> {
    // MaintenanceInfo is a public endpoint (auth middleware skips it, so no uid is ever inserted
    // into request extensions) -- the handler's own uid param is unused anyway. Extracting
    // web::ReqData<i64> here was failing (nothing to extract) and Actix was returning a 500
    // before this handler body ever ran, on every single call -- same bug as ServerNowTime.
    let req = parse_packet::<MaintenanceInfoRequest>("MaintenanceInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse MaintenanceInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = maintenance_info::handle(&pool, 0, req).await;
    Ok(HttpResponse::Ok().json(response))
}
