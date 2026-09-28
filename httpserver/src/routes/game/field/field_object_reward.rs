use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FieldObjectRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::field::field_object_reward;
use sqlx::SqlitePool;

#[put("FieldObjectReward")]
async fn field_object_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<FieldObjectRewardRequest>("FieldObjectReward", &body).map_err(|e| {
            tracing::warn!("Failed to parse FieldObjectReward: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = field_object_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
