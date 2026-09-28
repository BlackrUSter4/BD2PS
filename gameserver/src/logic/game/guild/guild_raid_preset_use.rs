use bd2::prost::Message;
use bd2::proto::proto_net::{GuildRaidPresetUseRequest, GuildRaidPresetUseResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::common::load_presets;

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildRaidPresetUseRequest) -> GameResponse {
    info!("Handling GuildRaidPresetUseRequest: {:?}", req);

    let presets = load_presets(pool, uid).await;
    let deck_info = presets
        .into_iter()
        .find(|p| p.slot == req.slot)
        .map(|p| p.deck_info.into_iter().filter_map(|d| d.deck_base_info).collect())
        .unwrap_or_default();

    let response = GuildRaidPresetUseResponse {
        deck_info,
        char_info: vec![],
        char_equip_info: vec![],
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
    let (route, code) = PacketCodeType::GuildRaidPresetUse.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
