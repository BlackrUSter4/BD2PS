use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LifeShopBuyInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::life::life_shop_buy_info;
use sqlx::SqlitePool;

#[put("LifeShopBuyInfo")]
async fn life_shop_buy_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LifeShopBuyInfoRequest>("LifeShopBuyInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse LifeShopBuyInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = life_shop_buy_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
