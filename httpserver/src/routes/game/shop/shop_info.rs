use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ShopInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::shop::shop_info;
use sqlx::SqlitePool;

#[put("ShopInfo")]
async fn shop_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ShopInfoRequest>("ShopInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse ShopInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = shop_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
