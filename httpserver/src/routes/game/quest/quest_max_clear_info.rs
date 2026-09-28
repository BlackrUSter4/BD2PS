use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::QuestMaxClearInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::quest::quest_max_clear_info;
use sqlx::SqlitePool;

#[put("QuestMaxClearInfo")]
async fn quest_max_clear_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<QuestMaxClearInfoRequest>("QuestMaxClearInfo", &body).map_err(|e| {
            tracing::warn!("Failed to parse QuestMaxClearInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = quest_max_clear_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
