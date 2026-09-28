use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FriendRemoveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::friend::friend_remove;
use sqlx::SqlitePool;

#[put("FriendRemove")]
async fn friend_remove_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FriendRemoveRequest>("FriendRemove", &body).map_err(|e| {
        tracing::warn!("Failed to parse FriendRemove: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = friend_remove::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
