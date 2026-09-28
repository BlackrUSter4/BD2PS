use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::SquareRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::square::square_reward;
use sqlx::SqlitePool;

#[put("SquareReward")]
async fn square_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<SquareRewardRequest>("SquareReward", &body).map_err(|e| {
        tracing::warn!("Failed to parse SquareReward: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = square_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
