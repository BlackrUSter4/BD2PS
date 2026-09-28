use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::UserLevelRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::user::user_level_reward;
use sqlx::SqlitePool;

#[put("UserLevelReward")]
async fn user_level_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<UserLevelRewardRequest>("UserLevelReward", &body).map_err(|e| {
        tracing::warn!("Failed to parse UserLevelReward: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = user_level_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
