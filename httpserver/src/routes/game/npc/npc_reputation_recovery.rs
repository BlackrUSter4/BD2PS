use actix_web::{HttpResponse, Result, put, web};
use bd2::NpcReputationRecoveryRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::npc::npc_reputation_recovery;
use sqlx::SqlitePool;

#[put("NpcReputationRecovery")]
async fn npc_reputation_recovery_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<NpcReputationRecoveryRequest>("NpcReputationRecovery", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse NpcReputationRecovery: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = npc_reputation_recovery::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
