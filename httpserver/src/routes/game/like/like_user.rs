use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LikeUserRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::like::like_user;
use sqlx::SqlitePool;

#[put("LikeUser")]
async fn like_user_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LikeUserRequest>("LikeUser", &body).map_err(|e| {
        tracing::warn!("Failed to parse LikeUser: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = like_user::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
