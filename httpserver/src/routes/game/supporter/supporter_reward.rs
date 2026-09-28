use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SupporterRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::supporter::supporter_reward;
use sqlx::SqlitePool;

#[put("SupporterReward")]
async fn supporter_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<SupporterRewardRequest>("SupporterReward", &body).map_err(|e| {
        tracing::warn!("Failed to parse SupporterReward: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = supporter_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
