use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildRaidSeasonRewardRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_raid_season_reward;
use sqlx::SqlitePool;

#[put("GuildRaidSeasonReward")]
async fn guild_raid_season_reward_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GuildRaidSeasonRewardRequest>("GuildRaidSeasonReward", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse GuildRaidSeasonReward: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = guild_raid_season_reward::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
