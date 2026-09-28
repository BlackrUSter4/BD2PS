use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PvpBattleOnceRewardInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pvp::pvp_battle_once_reward_info;
use sqlx::SqlitePool;

#[put("PvpBattleOnceRewardInfo")]
async fn pvp_battle_once_reward_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PvpBattleOnceRewardInfoRequest>("PvpBattleOnceRewardInfo", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse PvpBattleOnceRewardInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = pvp_battle_once_reward_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
