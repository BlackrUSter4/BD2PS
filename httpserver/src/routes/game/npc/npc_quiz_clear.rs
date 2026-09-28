use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::NpcQuizClearRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::npc::npc_quiz_clear;
use sqlx::SqlitePool;

#[put("NpcQuizClear")]
async fn npc_quiz_clear_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<NpcQuizClearRequest>("NpcQuizClear", &body).map_err(|e| {
        tracing::warn!("Failed to parse NpcQuizClear: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = npc_quiz_clear::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
