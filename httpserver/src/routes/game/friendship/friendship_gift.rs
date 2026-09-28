use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FriendshipGiftRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::friendship::friendship_gift;
use sqlx::SqlitePool;

#[put("FriendshipGift")]
async fn friendship_gift_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FriendshipGiftRequest>("FriendshipGift", &body).map_err(|e| {
        tracing::warn!("Failed to parse FriendshipGift: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = friendship_gift::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
