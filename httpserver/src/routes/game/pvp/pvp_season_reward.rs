use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PvpSeasonRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pvp::pvp_season_reward;
use sqlx::SqlitePool;

#[put("PvpSeasonReward")]
async fn pvp_season_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PvpSeasonRewardRequest>("PvpSeasonReward", &body).map_err(|e| {
        tracing::warn!("Failed to parse PvpSeasonReward: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = pvp_season_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
