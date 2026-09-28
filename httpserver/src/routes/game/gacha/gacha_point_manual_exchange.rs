use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GachaPointManualExchangeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::gacha::gacha_point_manual_exchange;
use sqlx::SqlitePool;

#[put("GachaPointManualExchange")]
async fn gacha_point_manual_exchange_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GachaPointManualExchangeRequest>("GachaPointManualExchange", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse GachaPointManualExchange: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = gacha_point_manual_exchange::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
