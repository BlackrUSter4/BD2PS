use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::QuestInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::quest::quest_info;
use sqlx::SqlitePool;

#[put("QuestInfo")]
async fn quest_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<QuestInfoRequest>("QuestInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse QuestInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = quest_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
