use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CancelLeaveUserRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::cancel::cancel_leave_user;
use sqlx::SqlitePool;

#[put("CancelLeaveUser")]
async fn cancel_leave_user_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CancelLeaveUserRequest>("CancelLeaveUser", &body).map_err(|e| {
        tracing::warn!("Failed to parse CancelLeaveUser: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = cancel_leave_user::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
