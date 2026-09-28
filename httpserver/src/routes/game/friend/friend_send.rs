use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FriendSendRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::friend::friend_send;
use sqlx::SqlitePool;

#[put("FriendSend")]
async fn friend_send_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FriendSendRequest>("FriendSend", &body).map_err(|e| {
        tracing::warn!("Failed to parse FriendSend: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = friend_send::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
