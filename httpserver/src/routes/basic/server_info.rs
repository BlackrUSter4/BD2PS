use actix_web::{HttpResponse, Result, put};
use bd2::proto::proto_net::ServerInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::server::server_info;

#[put("ServerInfo")]
async fn server_info_handler(body: String) -> Result<HttpResponse> {
    let req = parse_packet::<ServerInfoRequest>("ServerInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse ServerInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;

    let response = server_info::handle(req).await;
    Ok(HttpResponse::Ok().json(response))
}
