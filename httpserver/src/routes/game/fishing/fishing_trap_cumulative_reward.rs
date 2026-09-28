use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FishingTrapCumulativeRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::fishing::fishing_trap_cumulative_reward;
use sqlx::SqlitePool;

#[put("FishingTrapCumulativeReward")]
async fn fishing_trap_cumulative_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FishingTrapCumulativeRewardRequest>("FishingTrapCumulativeReward", &body).map_err(|e| {
        tracing::warn!("Failed to parse FishingTrapCumulativeReward: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = fishing_trap_cumulative_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
