use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::LifeHelperRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::life::life_helper_reward;
use sqlx::SqlitePool;

#[put("LifeHelperReward")]
async fn life_helper_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<LifeHelperRewardRequest>("LifeHelperReward", &body).map_err(|e| {
        tracing::warn!("Failed to parse LifeHelperReward: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = life_helper_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
