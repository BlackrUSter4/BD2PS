use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharAwakeInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char::char_awake_info;
use sqlx::SqlitePool;

#[put("CharAwakeInfo")]
async fn char_awake_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CharAwakeInfoRequest>("CharAwakeInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse CharAwakeInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = char_awake_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
