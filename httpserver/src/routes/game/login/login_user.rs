use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LoginUserRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::login::login_user;
use sqlx::SqlitePool;

#[put("LoginUser")]
async fn login_user_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LoginUserRequest>("LoginUser", &body).map_err(|e| {
        tracing::warn!("Failed to parse LoginUser: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = login_user::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
