use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::RefreshTokenRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::refresh::refresh_token;
use sqlx::SqlitePool;

#[put("RefreshToken")]
async fn refresh_token_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<RefreshTokenRequest>("RefreshToken", &body).map_err(|e| {
        tracing::warn!("Failed to parse RefreshToken: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = refresh_token::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
