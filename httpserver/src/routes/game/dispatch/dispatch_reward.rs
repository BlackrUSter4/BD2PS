use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::DispatchRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::dispatch::dispatch_reward;
use sqlx::SqlitePool;

#[put("DispatchReward")]
async fn dispatch_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<DispatchRewardRequest>("DispatchReward", &body).map_err(|e| {
        tracing::warn!("Failed to parse DispatchReward: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = dispatch_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
