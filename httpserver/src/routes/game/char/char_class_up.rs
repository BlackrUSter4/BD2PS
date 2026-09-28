use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharClassUpRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char::char_class_up;
use sqlx::SqlitePool;

#[put("CharClassUp")]
async fn char_class_up_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CharClassUpRequest>("CharClassUp", &body).map_err(|e| {
        tracing::warn!("Failed to parse CharClassUp: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = char_class_up::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
