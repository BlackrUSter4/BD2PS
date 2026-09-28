use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MyRoomLikeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::my::my_room_like;
use sqlx::SqlitePool;

#[put("MyRoomLike")]
async fn my_room_like_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MyRoomLikeRequest>("MyRoomLike", &body).map_err(|e| {
        tracing::warn!("Failed to parse MyRoomLike: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = my_room_like::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
