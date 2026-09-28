use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CommunityRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::community::community_reward;
use sqlx::SqlitePool;

#[put("CommunityReward")]
async fn community_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CommunityRewardRequest>("CommunityReward", &body).map_err(|e| {
        tracing::warn!("Failed to parse CommunityReward: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = community_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
