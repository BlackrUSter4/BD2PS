use bd2::prost::Message;
use bd2::proto::proto_net::{
    ColosseumPresetInfoRequest, ColosseumPresetInfoResponse, DeckDbInfo, PresetDbInfo, PresetDeckDbInfo,
    PresetDeckEquipDbInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::colosseum::colosseum_preset_info as preset_db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn build_preset_dbinfo(pool: &SqlitePool, preset: &database::models::game::colosseum::colosseum_preset_info::ColosseumPresetInfo) -> PresetDbInfo {
    let deck_rows = preset_db::get_deck(pool, preset.index).await.unwrap_or_default();
    let mut deck_info = Vec::with_capacity(deck_rows.len());
    for row in deck_rows {
        let equips = preset_db::get_deck_equips(pool, row.index).await.unwrap_or_default();
        deck_info.push(PresetDeckDbInfo {
            deck_base_info: Some(DeckDbInfo {
                char_inven_index: Some(row.char_inven_index),
                position: row.position,
                sequence: row.sequence,
            }),
            costume_inven_index: row.costume_inven_index,
            equip_info: equips
                .into_iter()
                .map(|e| PresetDeckEquipDbInfo { equip_type: e.equip_type, equip_inven_index: e.equip_inven_index })
                .collect(),
            team: row.team,
        });
    }
    PresetDbInfo {
        preset_name: preset.preset_name.clone(),
        preset_resource_id: preset.preset_resource_id,
        preset_resource_color: preset.preset_resource_color,
        slot: Some(preset.slot),
        deck_info,
    }
}

pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumPresetInfoRequest) -> GameResponse {
    info!("Handling ColosseumPresetInfoRequest: {:?}", req);

    let presets = preset_db::get_by_uid(pool, uid).await.unwrap_or_default();
    let mut preset_info = Vec::with_capacity(presets.len());
    for preset in &presets {
        preset_info.push(build_preset_dbinfo(pool, preset).await);
    }

    let response = ColosseumPresetInfoResponse { preset_info };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumPresetInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
