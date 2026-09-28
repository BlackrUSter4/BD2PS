use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MyRoomShopInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::my::my_room_shop_info;
use sqlx::SqlitePool;

#[put("MyRoomShopInfo")]
async fn my_room_shop_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MyRoomShopInfoRequest>("MyRoomShopInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse MyRoomShopInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = my_room_shop_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
