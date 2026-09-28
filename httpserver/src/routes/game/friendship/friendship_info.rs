use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FriendshipInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::friendship::friendship_info;
use sqlx::SqlitePool;

#[put("FriendshipInfo")]
async fn friendship_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FriendshipInfoRequest>("FriendshipInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse FriendshipInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = friendship_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
