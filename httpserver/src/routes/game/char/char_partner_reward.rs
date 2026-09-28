use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharPartnerRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char::char_partner_reward;
use sqlx::SqlitePool;

#[put("CharPartnerReward")]
async fn char_partner_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<CharPartnerRewardRequest>("CharPartnerReward", &body).map_err(|e| {
            tracing::warn!("Failed to parse CharPartnerReward: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = char_partner_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
