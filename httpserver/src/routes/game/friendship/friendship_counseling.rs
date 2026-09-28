use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FriendshipCounselingRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::friendship::friendship_counseling;
use sqlx::SqlitePool;

#[put("FriendshipCounseling")]
async fn friendship_counseling_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FriendshipCounselingRequest>("FriendshipCounseling", &body).map_err(|e| {
        tracing::warn!("Failed to parse FriendshipCounseling: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = friendship_counseling::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
