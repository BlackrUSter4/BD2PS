use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FriendReceiveListRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::friend::friend_receive_list;
use sqlx::SqlitePool;

#[put("FriendReceiveList")]
async fn friend_receive_list_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<FriendReceiveListRequest>("FriendReceiveList", &body).map_err(|e| {
            tracing::warn!("Failed to parse FriendReceiveList: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = friend_receive_list::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
