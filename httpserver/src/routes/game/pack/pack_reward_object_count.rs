use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PackRewardObjectCountRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pack::pack_reward_object_count;
use sqlx::SqlitePool;

#[put("PackRewardObjectCount")]
async fn pack_reward_object_count_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PackRewardObjectCountRequest>("PackRewardObjectCount", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse PackRewardObjectCount: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = pack_reward_object_count::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
