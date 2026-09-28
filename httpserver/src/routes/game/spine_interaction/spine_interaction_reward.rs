use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SpineInteractionRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::spine_interaction::spine_interaction_reward;
use sqlx::SqlitePool;

#[put("SpineInteractionReward")]
async fn spine_interaction_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<SpineInteractionRewardRequest>("SpineInteractionReward", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse SpineInteractionReward: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = spine_interaction_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
