use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CommunityRewardInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::community::community_reward_info;
use sqlx::SqlitePool;

#[put("CommunityRewardInfo")]
async fn community_reward_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<CommunityRewardInfoRequest>("CommunityRewardInfo", &body).map_err(|e| {
            tracing::warn!("Failed to parse CommunityRewardInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = community_reward_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
