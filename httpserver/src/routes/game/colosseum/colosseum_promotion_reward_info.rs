use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ColosseumPromotionRewardInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::colosseum::colosseum_promotion_reward_info;
use sqlx::SqlitePool;

#[put("ColosseumPromotionRewardInfo")]
async fn colosseum_promotion_reward_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ColosseumPromotionRewardInfoRequest>("ColosseumPromotionRewardInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse ColosseumPromotionRewardInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = colosseum_promotion_reward_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
