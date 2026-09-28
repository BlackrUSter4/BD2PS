use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharAllRevivalRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char::char_all_revival;
use sqlx::SqlitePool;

#[put("CharAllRevival")]
async fn char_all_revival_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CharAllRevivalRequest>("CharAllRevival", &body).map_err(|e| {
        tracing::warn!("Failed to parse CharAllRevival: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = char_all_revival::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
