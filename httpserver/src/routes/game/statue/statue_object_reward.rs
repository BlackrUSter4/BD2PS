use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::StatueObjectRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::statue::statue_object_reward;
use sqlx::SqlitePool;

#[put("StatueObjectReward")]
async fn statue_object_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<StatueObjectRewardRequest>("StatueObjectReward", &body).map_err(|e| {
            tracing::warn!("Failed to parse StatueObjectReward: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = statue_object_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
