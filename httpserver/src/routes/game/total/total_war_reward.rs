use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::TotalWarRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::total::total_war_reward;
use sqlx::SqlitePool;

#[put("TotalWarReward")]
async fn total_war_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<TotalWarRewardRequest>("TotalWarReward", &body).map_err(|e| {
        tracing::warn!("Failed to parse TotalWarReward: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = total_war_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
