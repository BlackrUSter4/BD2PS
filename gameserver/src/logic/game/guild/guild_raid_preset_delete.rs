use bd2::prost::Message;
use bd2::proto::proto_net::{GuildRaidPresetDeleteRequest, GuildRaidPresetDeleteResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::common::{load_presets, save_presets};

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildRaidPresetDeleteRequest) -> GameResponse {
    info!("Handling GuildRaidPresetDeleteRequest: {:?}", req);

    let mut presets = load_presets(pool, uid).await;
    presets.retain(|p| !req.slot.contains(&p.slot.unwrap_or(-1)));
    save_presets(pool, uid, &presets).await;

    let response = GuildRaidPresetDeleteResponse {};
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
    // No dedicated PacketCodeType exists for this route (verified against
    // common/src/packet_code.rs) — Common is the established fallback the
    // original scaffolding already used for the sibling PresetInfoChange route.
    let (route, code) = PacketCodeType::Common.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
