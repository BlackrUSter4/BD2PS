use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PassRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pass::pass_reward;
use sqlx::SqlitePool;

#[put("PassReward")]
async fn pass_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PassRewardRequest>("PassReward", &body).map_err(|e| {
        tracing::warn!("Failed to parse PassReward: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = pass_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
