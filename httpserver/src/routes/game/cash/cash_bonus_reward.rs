use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CashBonusRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::cash::cash_bonus_reward;
use sqlx::SqlitePool;

#[put("CashBonusReward")]
async fn cash_bonus_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CashBonusRewardRequest>("CashBonusReward", &body).map_err(|e| {
        tracing::warn!("Failed to parse CashBonusReward: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = cash_bonus_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
