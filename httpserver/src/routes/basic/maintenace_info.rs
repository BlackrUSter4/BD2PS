use actix_web::{HttpResponse, Result, put};
use bd2::proto::proto_net::MaintenanceInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::maintenace::maintenace_info;

#[put("MaintenanceInfo")]
async fn maintenance_info_handler(body: String) -> Result<HttpResponse> {
    let req = parse_packet::<MaintenanceInfoRequest>("MaintenanceInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse MaintenanceInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;

    let response = maintenace_info::handle(req).await;
    Ok(HttpResponse::Ok().json(response))
}
