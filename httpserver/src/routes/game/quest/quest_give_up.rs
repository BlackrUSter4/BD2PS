use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::QuestGiveUpRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::quest::quest_give_up;
use sqlx::SqlitePool;

#[put("QuestGiveUp")]
async fn quest_give_up_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<QuestGiveUpRequest>("QuestGiveUp", &body).map_err(|e| {
        tracing::warn!("Failed to parse QuestGiveUp: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = quest_give_up::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
