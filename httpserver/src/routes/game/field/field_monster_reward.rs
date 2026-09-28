use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FieldMonsterRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::field::field_monster_reward;
use sqlx::SqlitePool;

#[put("FieldMonsterReward")]
async fn field_monster_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<FieldMonsterRewardRequest>("FieldMonsterReward", &body).map_err(|e| {
            tracing::warn!("Failed to parse FieldMonsterReward: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = field_monster_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
