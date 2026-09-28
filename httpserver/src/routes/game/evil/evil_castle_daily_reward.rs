use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EvilCastleDailyRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::evil::evil_castle_daily_reward;
use sqlx::SqlitePool;

#[put("EvilCastleDailyReward")]
async fn evil_castle_daily_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EvilCastleDailyRewardRequest>("EvilCastleDailyReward", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse EvilCastleDailyReward: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = evil_castle_daily_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
