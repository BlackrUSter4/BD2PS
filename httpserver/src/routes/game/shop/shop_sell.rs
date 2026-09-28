use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ShopSellRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::shop::shop_sell;
use sqlx::SqlitePool;

#[put("ShopSell")]
async fn shop_sell_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ShopSellRequest>("ShopSell", &body).map_err(|e| {
        tracing::warn!("Failed to parse ShopSell: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = shop_sell::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
