use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ClearPackageRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::clear::clear_package_reward;
use sqlx::SqlitePool;

#[put("ClearPackageReward")]
async fn clear_package_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<ClearPackageRewardRequest>("ClearPackageReward", &body).map_err(|e| {
            tracing::warn!("Failed to parse ClearPackageReward: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = clear_package_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
