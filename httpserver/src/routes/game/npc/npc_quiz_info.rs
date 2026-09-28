use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::NpcQuizInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::npc::npc_quiz_info;
use sqlx::SqlitePool;

#[put("NpcQuizInfo")]
async fn npc_quiz_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<NpcQuizInfoRequest>("NpcQuizInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse NpcQuizInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = npc_quiz_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
