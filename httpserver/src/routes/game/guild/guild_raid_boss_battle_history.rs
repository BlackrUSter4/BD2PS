use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildRaidBossBattleHistoryRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_raid_boss_battle_history;
use sqlx::SqlitePool;

#[put("GuildRaidBossBattleHistory")]
async fn guild_raid_boss_battle_history_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<GuildRaidBossBattleHistoryRequest>("GuildRaidBossBattleHistory", &body)
            .map_err(|e| {
                tracing::warn!("Failed to parse GuildRaidBossBattleHistory: {}", e);
                actix_web::error::ErrorBadRequest("Invalid packet")
            })?;
    let response = guild_raid_boss_battle_history::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
