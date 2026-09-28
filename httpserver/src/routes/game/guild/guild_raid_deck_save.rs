use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildRaidDeckSaveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_raid_deck_save;
use sqlx::SqlitePool;

#[put("GuildRaidDeckSave")]
async fn guild_raid_deck_save_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<GuildRaidDeckSaveRequest>("GuildRaidDeckSave", &body).map_err(|e| {
            tracing::warn!("Failed to parse GuildRaidDeckSave: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = guild_raid_deck_save::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
