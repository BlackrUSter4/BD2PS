use bd2::prost::Message;
use bd2::proto::proto_net::{GuildRaidPresetInfoChangeRequest, GuildRaidPresetInfoChangeResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::common::{load_presets, save_presets};

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildRaidPresetInfoChangeRequest) -> GameResponse {
    info!("Handling GuildRaidPresetInfoChangeRequest: {:?}", req);

    let mut presets = load_presets(pool, uid).await;
    if let Some(p) = presets.iter_mut().find(|p| p.slot == req.slot) {
        if req.preset_name.is_some() {
            p.preset_name = req.preset_name.clone();
        }
        if req.preset_resource_id.is_some() {
            p.preset_resource_id = req.preset_resource_id;
        }
        if req.preset_resource_color.is_some() {
            p.preset_resource_color = req.preset_resource_color;
        }
        save_presets(pool, uid, &presets).await;
    }

    let response = GuildRaidPresetInfoChangeResponse {};
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
    let (route, code) = PacketCodeType::Common.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
