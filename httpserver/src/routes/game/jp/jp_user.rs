use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::JpUserRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::jp::jp_user;
use sqlx::SqlitePool;

#[put("JpUser")]
async fn jp_user_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<JpUserRequest>("JpUser", &body).map_err(|e| {
        tracing::warn!("Failed to parse JpUser: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = jp_user::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
