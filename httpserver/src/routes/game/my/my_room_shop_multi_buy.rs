use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MyRoomShopMultiBuyRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::my::my_room_shop_multi_buy;
use sqlx::SqlitePool;

#[put("MyRoomShopMultiBuy")]
async fn my_room_shop_multi_buy_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MyRoomShopMultiBuyRequest>("MyRoomShopMultiBuy", &body).map_err(|e| {
        tracing::warn!("Failed to parse MyRoomShopMultiBuy: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = my_room_shop_multi_buy::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
