use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GuildRaidPresetSlotAddRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::guild::guild_raid_preset_slot_add;
use sqlx::SqlitePool;

#[put("GuildRaidPresetSlotAdd")]
async fn guild_raid_preset_slot_add_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GuildRaidPresetSlotAddRequest>("GuildRaidPresetSlotAdd", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse GuildRaidPresetSlotAdd: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = guild_raid_preset_slot_add::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
