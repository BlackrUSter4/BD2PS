use bd2::prost::Message;
use bd2::proto::proto_net::{MonsterHuntPresetSaveRequest, MonsterHuntPresetSaveResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::monster::monster_hunt_preset_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: MonsterHuntPresetSaveRequest,
) -> GameResponse {
    info!("Handling MonsterHuntPresetSaveRequest: {:?}", req);

    if let Some(preset) = &req.preset_info {
        let slot = preset.slot.unwrap_or(0);
        let json = serde_json::to_string(&preset.deck_info).ok();
        let _ = db::save_slot(
            pool,
            uid,
            slot,
            preset.preset_name.as_deref(),
            preset.preset_resource_id,
            preset.preset_resource_color,
            json.as_deref(),
        )
        .await;
    }

    let response = MonsterHuntPresetSaveResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MonsterHuntPresetSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
