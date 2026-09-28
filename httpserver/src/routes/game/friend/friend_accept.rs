use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FriendAcceptRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::friend::friend_accept;
use sqlx::SqlitePool;

#[put("FriendAccept")]
async fn friend_accept_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FriendAcceptRequest>("FriendAccept", &body).map_err(|e| {
        tracing::warn!("Failed to parse FriendAccept: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = friend_accept::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
