use actix_web::{HttpResponse, Result, put};
use bd2::proto::proto_net::BalanceVersionCheckRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::balance::balance_version_check;

#[put("BalanceVersionCheck")]
async fn balance_check_version_handler(body: String) -> Result<HttpResponse> {
    let req =
        parse_packet::<BalanceVersionCheckRequest>("BalanceVersionCheck", &body).map_err(|e| {
            tracing::warn!("Failed to parse BalanceCheckVersion: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;

    let response = balance_version_check::handle(req).await;
    Ok(HttpResponse::Ok().json(response))
}
