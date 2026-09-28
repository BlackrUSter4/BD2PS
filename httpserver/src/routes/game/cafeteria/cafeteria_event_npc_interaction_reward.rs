use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CafeteriaEventNpcInteractionRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::cafeteria::cafeteria_event_npc_interaction_reward;
use sqlx::SqlitePool;

#[put("CafeteriaEventNpcInteractionReward")]
async fn cafeteria_event_npc_interaction_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CafeteriaEventNpcInteractionRewardRequest>(
        "CafeteriaEventNpcInteractionReward",
        &body,
    )
    .map_err(|e| {
        tracing::warn!("Failed to parse CafeteriaEventNpcInteractionReward: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = cafeteria_event_npc_interaction_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
