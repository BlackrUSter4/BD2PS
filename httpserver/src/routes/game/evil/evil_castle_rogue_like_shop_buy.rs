use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EvilCastleRogueLikeShopBuyRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::evil::evil_castle_rogue_like_shop_buy;
use sqlx::SqlitePool;

#[put("EvilCastleRogueLikeShopBuy")]
async fn evil_castle_rogue_like_shop_buy_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<EvilCastleRogueLikeShopBuyRequest>("EvilCastleRogueLikeShopBuy", &body)
            .map_err(|e| {
                tracing::warn!("Failed to parse EvilCastleRogueLikeShopBuy: {}", e);
                actix_web::error::ErrorBadRequest("Invalid packet")
            })?;
    let response = evil_castle_rogue_like_shop_buy::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
