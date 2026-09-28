use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameFieldRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_field_reward;
use sqlx::SqlitePool;

#[put("MiniGameFieldReward")]
async fn mini_game_field_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<MiniGameFieldRewardRequest>("MiniGameFieldReward", &body).map_err(|e| {
            tracing::warn!("Failed to parse MiniGameFieldReward: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = mini_game_field_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
