use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::QuestClearRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::quest::quest_clear;
use sqlx::SqlitePool;

#[put("QuestClear")]
async fn quest_clear_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<QuestClearRequest>("QuestClear", &body).map_err(|e| {
        tracing::warn!("Failed to parse QuestClear: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = quest_clear::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
