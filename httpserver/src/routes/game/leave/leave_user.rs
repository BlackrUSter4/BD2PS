use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LeaveUserRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::leave::leave_user;
use sqlx::SqlitePool;

#[put("LeaveUser")]
async fn leave_user_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LeaveUserRequest>("LeaveUser", &body).map_err(|e| {
        tracing::warn!("Failed to parse LeaveUser: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = leave_user::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
