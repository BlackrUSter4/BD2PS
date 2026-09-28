use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ColosseumSeasonRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::colosseum::colosseum_season_reward;
use sqlx::SqlitePool;

#[put("ColosseumSeasonReward")]
async fn colosseum_season_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ColosseumSeasonRewardRequest>("ColosseumSeasonReward", &body).map_err(|e| {
        tracing::warn!("Failed to parse ColosseumSeasonReward: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = colosseum_season_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
