use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::UserNicknameChangeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::user::user_nickname_change;
use sqlx::SqlitePool;

#[put("UserNicknameChange")]
async fn user_nickname_change_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<UserNicknameChangeRequest>("UserNicknameChange", &body).map_err(|e| {
            tracing::warn!("Failed to parse UserNicknameChange: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = user_nickname_change::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
