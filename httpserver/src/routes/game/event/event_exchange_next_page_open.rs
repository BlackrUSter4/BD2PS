use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EventExchangeNextPageOpenRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::event::event_exchange_next_page_open;
use sqlx::SqlitePool;

#[put("EventExchangeNextPageOpen")]
async fn event_exchange_next_page_open_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EventExchangeNextPageOpenRequest>("EventExchangeNextPageOpen", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse EventExchangeNextPageOpen: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = event_exchange_next_page_open::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
