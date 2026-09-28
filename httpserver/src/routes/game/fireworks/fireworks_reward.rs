use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FireWorksRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::fireworks::fireworks_reward;
use sqlx::SqlitePool;

#[put("FireWorksReward")]
async fn fireworks_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FireWorksRewardRequest>("FireWorksReward", &body).map_err(|e| {
        tracing::warn!("Failed to parse FireWorksReward: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = fireworks_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
