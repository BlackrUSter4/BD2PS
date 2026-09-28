use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::JoinUserRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::join::join_user;
use sqlx::SqlitePool;

#[put("JoinUser")]
async fn join_user_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<JoinUserRequest>("JoinUser", &body).map_err(|e| {
        tracing::warn!("Failed to parse JoinUser: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = join_user::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
