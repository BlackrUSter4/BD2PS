use bd2::prost::Message;
use bd2::proto::proto_net::{TotalWarDeckPresetSaveRequest, TotalWarDeckPresetSaveResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::total::total_war_deck_preset_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: TotalWarDeckPresetSaveRequest,
) -> GameResponse {
    info!("Handling TotalWarDeckPresetSaveRequest: {:?}", req);

    if let Some(preset) = &req.preset_info {
        let slot = preset.slot.unwrap_or(0);
        let json = serde_json::to_string(&preset.deck_info).ok();
        let _ = db::save_slot(
            pool,
            uid,
            slot,
            preset.preset_name.as_deref(),
            preset.resource_id,
            preset.resource_color,
            json.as_deref(),
        )
        .await;
    }

    let response = TotalWarDeckPresetSaveResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::TotalWarDeckPresetSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
