use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FishingShopSellRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::fishing::fishing_shop_sell;
use sqlx::SqlitePool;

#[put("FishingShopSell")]
async fn fishing_shop_sell_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FishingShopSellRequest>("FishingShopSell", &body).map_err(|e| {
        tracing::warn!("Failed to parse FishingShopSell: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = fishing_shop_sell::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
