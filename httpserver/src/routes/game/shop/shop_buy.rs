use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ShopBuyRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::shop::shop_buy;
use sqlx::SqlitePool;

#[put("ShopBuy")]
async fn shop_buy_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ShopBuyRequest>("ShopBuy", &body).map_err(|e| {
        tracing::warn!("Failed to parse ShopBuy: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = shop_buy::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
