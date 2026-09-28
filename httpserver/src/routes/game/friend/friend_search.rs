use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FriendSearchRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::friend::friend_search;
use sqlx::SqlitePool;

#[put("FriendSearch")]
async fn friend_search_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FriendSearchRequest>("FriendSearch", &body).map_err(|e| {
        tracing::warn!("Failed to parse FriendSearch: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = friend_search::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
