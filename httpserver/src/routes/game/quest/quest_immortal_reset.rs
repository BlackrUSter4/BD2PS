use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::QuestImmortalResetRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::quest::quest_immortal_reset;
use sqlx::SqlitePool;

#[put("QuestImmortalReset")]
async fn quest_immortal_reset_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<QuestImmortalResetRequest>("QuestImmortalReset", &body).map_err(|e| {
            tracing::warn!("Failed to parse QuestImmortalReset: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = quest_immortal_reset::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
