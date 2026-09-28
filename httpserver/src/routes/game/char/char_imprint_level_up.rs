use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharImprintLevelUpRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char::char_imprint_level_up;
use sqlx::SqlitePool;

#[put("CharImprintLevelUp")]
async fn char_imprint_level_up_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<CharImprintLevelUpRequest>("CharImprintLevelUp", &body).map_err(|e| {
            tracing::warn!("Failed to parse CharImprintLevelUp: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = char_imprint_level_up::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
