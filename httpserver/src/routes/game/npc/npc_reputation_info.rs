use actix_web::{HttpResponse, Result, put, web};
use bd2::NpcReputationInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::npc::npc_reputation_info;
use sqlx::SqlitePool;

#[put("NpcReputationInfo")]
async fn npc_reputation_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<NpcReputationInfoRequest>("NpcReputationInfo", &body).map_err(|e| {
            tracing::warn!("Failed to parse NpcReputationInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = npc_reputation_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
