use bd2::prost::Message;
use bd2::proto::proto_net::{GuildRaidDeckSaveRequest, GuildRaidDeckSaveResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::guild_raid_deck_info;
use database::models::game::guild::guild_raid_deck_info::GuildRaidDeckInfo;
use sqlx::SqlitePool;
use tracing::info;

/// The scaffolded schema stores decks as opaque TEXT columns rather than a
/// normalized child table — repurposed here to hold serialized JSON directly
/// (round-trips correctly, doesn't try to honor whatever indirection the
/// original "Index" naming implied).
pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildRaidDeckSaveRequest) -> GameResponse {
    info!("Handling GuildRaidDeckSaveRequest: {:?}", req);

    let _ = guild_raid_deck_info::delete_guild_raid_deck_info(pool, uid).await;
    let row = GuildRaidDeckInfo {
        index: 0,
        uid,
        deck_info_index: Some(serde_json::to_string(&req.deck_info).unwrap_or_default()),
        supporter_deck_info_index: Some(serde_json::to_string(&req.supporter_deck_info).unwrap_or_default()),
        is_supporter_deck_update: Some(if req.supporter_deck_info.is_empty() { 0 } else { 1 }),
    };
    if let Err(e) = guild_raid_deck_info::add_guild_raid_deck_info(pool, &row).await {
        tracing::warn!("GuildRaidDeckSave: failed to save: {}", e);
    }

    let response = GuildRaidDeckSaveResponse {
        is_supporter_update: row.is_supporter_deck_update.map(|v| v != 0),
    };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::GuildRaidDeckSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
