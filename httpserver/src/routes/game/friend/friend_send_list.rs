use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FriendSendListRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::friend::friend_send_list;
use sqlx::SqlitePool;

#[put("FriendSendList")]
async fn friend_send_list_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FriendSendListRequest>("FriendSendList", &body).map_err(|e| {
        tracing::warn!("Failed to parse FriendSendList: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = friend_send_list::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
