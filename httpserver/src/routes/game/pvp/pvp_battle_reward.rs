use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PvpBattleRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pvp::pvp_battle_reward;
use sqlx::SqlitePool;

#[put("PvpBattleReward")]
async fn pvp_battle_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PvpBattleRewardRequest>("PvpBattleReward", &body).map_err(|e| {
        tracing::warn!("Failed to parse PvpBattleReward: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = pvp_battle_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
