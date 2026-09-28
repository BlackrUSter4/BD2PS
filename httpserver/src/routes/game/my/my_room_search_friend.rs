use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MyRoomSearchFriendRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::my::my_room_search_friend;
use sqlx::SqlitePool;

#[put("MyRoomSearchFriend")]
async fn my_room_search_friend_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<MyRoomSearchFriendRequest>("MyRoomSearchFriend", &body).map_err(|e| {
            tracing::warn!("Failed to parse MyRoomSearchFriend: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = my_room_search_friend::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
