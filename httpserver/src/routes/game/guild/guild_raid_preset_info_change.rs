use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildRaidPresetInfoChangeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_raid_preset_info_change;
use sqlx::SqlitePool;

#[put("GuildRaidPresetInfoChange")]
async fn guild_raid_preset_info_change_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GuildRaidPresetInfoChangeRequest>("GuildRaidPresetInfoChange", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse GuildRaidPresetInfoChange: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = guild_raid_preset_info_change::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
