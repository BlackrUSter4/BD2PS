use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::HuntDispatchRewardPreviewRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::hunt::hunt_dispatch_reward_preview;
use sqlx::SqlitePool;

#[put("HuntDispatchRewardPreview")]
async fn hunt_dispatch_reward_preview_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<HuntDispatchRewardPreviewRequest>("HuntDispatchRewardPreview", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse HuntDispatchRewardPreview: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = hunt_dispatch_reward_preview::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
