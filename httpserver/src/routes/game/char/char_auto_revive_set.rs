use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharAutoReviveSetRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char::char_auto_revive_set;
use sqlx::SqlitePool;

#[put("CharAutoReviveSet")]
async fn char_auto_revive_set_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<CharAutoReviveSetRequest>("CharAutoReviveSet", &body).map_err(|e| {
            tracing::warn!("Failed to parse CharAutoReviveSet: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = char_auto_revive_set::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
