use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EventExchangeInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::event::event_exchange_info;
use sqlx::SqlitePool;

#[put("EventExchangeInfo")]
async fn event_exchange_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<EventExchangeInfoRequest>("EventExchangeInfo", &body).map_err(|e| {
            tracing::warn!("Failed to parse EventExchangeInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = event_exchange_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
