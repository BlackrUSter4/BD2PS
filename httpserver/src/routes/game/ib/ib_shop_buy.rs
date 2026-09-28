use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::IbShopBuyRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::ib::ib_shop_buy;
use sqlx::SqlitePool;

#[put("IbShopBuy")]
async fn ib_shop_buy_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<IbShopBuyRequest>("IbShopBuy", &body).map_err(|e| {
        tracing::warn!("Failed to parse IbShopBuy: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = ib_shop_buy::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
