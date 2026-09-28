use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildRaidSeasonRankingRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_raid_season_ranking;
use sqlx::SqlitePool;

#[put("GuildRaidSeasonRanking")]
async fn guild_raid_season_ranking_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GuildRaidSeasonRankingRequest>("GuildRaidSeasonRanking", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse GuildRaidSeasonRanking: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = guild_raid_season_ranking::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
