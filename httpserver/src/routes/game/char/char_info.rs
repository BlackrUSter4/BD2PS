use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char::char_info;
use sqlx::SqlitePool;

#[put("CharInfo")]
async fn char_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CharInfoRequest>("CharInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse CharInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = char_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
