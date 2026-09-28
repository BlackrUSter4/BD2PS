use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::DatingMessageUpdateRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::dating::dating_message_update;
use sqlx::SqlitePool;

#[put("DatingMessageUpdate")]
async fn dating_message_update_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<DatingMessageUpdateRequest>("DatingMessageUpdate", &body).map_err(|e| {
            tracing::warn!("Failed to parse DatingMessageUpdate: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = dating_message_update::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
