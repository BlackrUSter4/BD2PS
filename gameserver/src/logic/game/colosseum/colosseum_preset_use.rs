use bd2::prost::Message;
use bd2::proto::proto_net::{
    ColosseumDeckInfo as DeckInfoProto, ColosseumPresetUseRequest, ColosseumPresetUseResponse,
    ContentsCharItemInfo, ContentsEquipDbInfo, PresetDeckBlessDbInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::colosseum::{colosseum_bless_info, colosseum_preset_info as preset_db};
use sqlx::SqlitePool;
use tracing::info;

use super::{build_deck_info_list, default_notify, save_deck};

/// Presets don't store their own bless loadout — bless is a separate, deck-type-scoped
/// setting (`ColosseumBlessSaveRequest`) with no per-preset save endpoint in the schema, so
/// "using" a preset applies its deck and simply echoes back whatever bless is currently
/// active (real, current state — not preset-specific, since there's nothing preset-specific
/// to read).
pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumPresetUseRequest) -> GameResponse {
    info!("Handling ColosseumPresetUseRequest: {:?}", req);

    let slot = req.slot.unwrap_or(0);
    if let Some(preset) = preset_db::get_by_uid_and_slot(pool, uid, slot).await.ok().flatten() {
        let deck_rows = preset_db::get_deck(pool, preset.index).await.unwrap_or_default();
        let mut deck_info = Vec::with_capacity(deck_rows.len());
        let mut items = Vec::with_capacity(deck_rows.len());
        for row in &deck_rows {
            deck_info.push(DeckInfoProto {
                char_inven_index: Some(row.char_inven_index),
                position: row.position,
                sequence: row.sequence,
                costume_inven_index: row.costume_inven_index,
            });
            let equips = preset_db::get_deck_equips(pool, row.index).await.unwrap_or_default();
            items.push(ContentsCharItemInfo {
                char_inven_index: Some(row.char_inven_index),
                equip_info: equips
                    .into_iter()
                    .map(|e| ContentsEquipDbInfo { equip_inven_index: e.equip_inven_index, equip_type: e.equip_type })
                    .collect(),
                connect_potential_costume: None,
            });
        }
        let _ = save_deck(pool, uid, &deck_info, &items).await;
    }

    let deck_info = build_deck_info_list(pool, uid).await;
    let bless_rows = colosseum_bless_info::get_by_uid(pool, uid).await.unwrap_or_default();
    let bless_info = vec![
        PresetDeckBlessDbInfo {
            deck_type: Some(0),
            id: bless_rows.iter().filter(|r| r.deck_type == 0).map(|r| r.bless_id).collect(),
        },
        PresetDeckBlessDbInfo {
            deck_type: Some(1),
            id: bless_rows.iter().filter(|r| r.deck_type == 1).map(|r| r.bless_id).collect(),
        },
    ];

    let response = ColosseumPresetUseResponse { deck_info, bless_info };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumPresetUse.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
