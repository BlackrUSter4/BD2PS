use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildRaidDeckInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_raid_deck_info;
use sqlx::SqlitePool;

#[put("GuildRaidDeckInfo")]
async fn guild_raid_deck_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<GuildRaidDeckInfoRequest>("GuildRaidDeckInfo", &body).map_err(|e| {
            tracing::warn!("Failed to parse GuildRaidDeckInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = guild_raid_deck_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
