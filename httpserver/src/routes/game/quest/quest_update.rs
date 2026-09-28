use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::QuestUpdateRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::quest::quest_update;
use sqlx::SqlitePool;

#[put("QuestUpdate")]
async fn quest_update_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<QuestUpdateRequest>("QuestUpdate", &body).map_err(|e| {
        tracing::warn!("Failed to parse QuestUpdate: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = quest_update::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
