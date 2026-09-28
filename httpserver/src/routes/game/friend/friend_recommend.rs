use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FriendRecommendRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::friend::friend_recommend;
use sqlx::SqlitePool;

#[put("FriendRecommend")]
async fn friend_recommend_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FriendRecommendRequest>("FriendRecommend", &body).map_err(|e| {
        tracing::warn!("Failed to parse FriendRecommend: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = friend_recommend::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
