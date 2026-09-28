use bd2::prost::Message;
use bd2::proto::proto_net::{GuildRaidPresetSlotAddRequest, GuildRaidPresetSlotAddResponse, Notify, PresetDbInfo};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::common::{load_presets, save_presets};

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildRaidPresetSlotAddRequest) -> GameResponse {
    info!("Handling GuildRaidPresetSlotAddRequest: {:?}", req);

    let mut presets = load_presets(pool, uid).await;
    let mut next_slot = presets.iter().filter_map(|p| p.slot).max().unwrap_or(0) + 1;
    for _ in 0..req.add_count.unwrap_or(1).max(1) {
        presets.push(PresetDbInfo {
            preset_name: Some(String::new()),
            preset_resource_id: None,
            preset_resource_color: None,
            slot: Some(next_slot),
            deck_info: vec![],
        });
        next_slot += 1;
    }
    save_presets(pool, uid, &presets).await;

    let response = GuildRaidPresetSlotAddResponse {};
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
    let (route, code) = PacketCodeType::GuildRaidPresetSlotAdd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
