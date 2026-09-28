use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharPartnerStoryRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char::char_partner_story_reward;
use sqlx::SqlitePool;

#[put("CharPartnerStoryReward")]
async fn char_partner_story_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CharPartnerStoryRewardRequest>("CharPartnerStoryReward", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse CharPartnerStoryReward: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = char_partner_story_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
