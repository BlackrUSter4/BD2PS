use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CafeteriaIntroductionStoryRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::cafeteria::cafeteria_introduction_story_reward;
use sqlx::SqlitePool;

#[put("CafeteriaIntroductionStoryReward")]
async fn cafeteria_introduction_story_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CafeteriaIntroductionStoryRewardRequest>(
        "CafeteriaIntroductionStoryReward",
        &body,
    )
    .map_err(|e| {
        tracing::warn!("Failed to parse CafeteriaIntroductionStoryReward: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = cafeteria_introduction_story_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
