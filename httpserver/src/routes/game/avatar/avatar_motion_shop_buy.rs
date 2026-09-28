use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::AvatarMotionShopBuyRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::avatar::avatar_motion_shop_buy;
use sqlx::SqlitePool;

#[put("AvatarMotionShopBuy")]
async fn avatar_motion_shop_buy_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<AvatarMotionShopBuyRequest>("AvatarMotionShopBuy", &body).map_err(|e| {
        tracing::warn!("Failed to parse AvatarMotionShopBuy: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = avatar_motion_shop_buy::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
