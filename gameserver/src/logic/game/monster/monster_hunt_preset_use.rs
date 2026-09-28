use bd2::prost::Message;
use bd2::proto::proto_net::{
    CharDbInfo, MonsterHuntDeckDbInfo, MonsterHuntPresetUseRequest, MonsterHuntPresetUseResponse,
    PresetDeckDbInfo, PresetUseEquipInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{
    char::char_info,
    equip::equip_info,
    monster::{monster_hunt_deck_info as deck_db, monster_hunt_preset_info as db},
};
use sqlx::SqlitePool;
use std::collections::HashMap;
use tracing::info;

use super::default_notify;

/// "Use" a preset: apply its saved deck-per-team layout onto the account's live
/// MonsterHuntDeckInfo rows (same effect as manually re-saving each team via
/// MonsterHuntDeckSave). `char_info`/`char_equip_info` are built for real from the account's
/// actual CharInfo/EquipInfo rows for every char_inven_index that ends up in the applied deck
/// (same technique as the generic Preset system's `preset_use.rs`, adapted since MonsterHunt
/// has no per-deck-row equip storage of its own — reads whatever's *currently* equipped on
/// each character instead, which is the real answer to "what does this character have on").
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: MonsterHuntPresetUseRequest,
) -> GameResponse {
    info!("Handling MonsterHuntPresetUseRequest: {:?}", req);

    let slot = req.slot.unwrap_or(0);
    let mut monster_hunt_deck_info = Vec::new();
    let mut char_inven_indices: Vec<i64> = Vec::new();

    if let Ok(Some(preset)) = db::get_by_uid_and_slot(pool, uid, slot).await {
        if let Some(json) = preset.preset_info_index.as_deref() {
            if let Ok(deck_slots) = serde_json::from_str::<Vec<PresetDeckDbInfo>>(json) {
                let mut by_team: HashMap<i32, Vec<bd2::proto::proto_net::DeckDbInfo>> =
                    HashMap::new();
                for slot_info in deck_slots {
                    let team = slot_info.team.unwrap_or(0);
                    if let Some(base) = slot_info.deck_base_info {
                        by_team.entry(team).or_default().push(base);
                    }
                }
                for (team, decks) in by_team {
                    char_inven_indices.extend(decks.iter().filter_map(|d| d.char_inven_index));
                    let json = serde_json::to_string(&decks).ok();
                    let _ = deck_db::upsert(pool, uid, team, json.as_deref(), None).await;
                    monster_hunt_deck_info.push(MonsterHuntDeckDbInfo {
                        team: Some(team),
                        deck_info: decks,
                        battle_power: None,
                    });
                }
            }
        }
    }

    char_inven_indices.sort_unstable();
    char_inven_indices.dedup();

    let mut char_info = Vec::with_capacity(char_inven_indices.len());
    let mut char_equip_info = Vec::with_capacity(char_inven_indices.len());
    for idx in char_inven_indices {
        if let Ok(Some(row)) = char_info::get_by_inven_index(pool, uid, idx).await {
            char_info.push(CharDbInfo {
                inven_index: row.inven_index,
                id: row.id,
                hp: row.hp,
                level: row.level,
                costume_id: row.costume_id,
                exp: row.exp,
                use_costume: row.use_costume,
                talent_level: row.talent_level,
                talent_exp: row.talent_exp,
                solidarity_reward: row.solidarity_reward,
                expiry_time: row.expiry_time,
                pictorialbook_info: vec![],
                connect_potential_costume: row.connect_potential_costume,
            });
        }

        if let Ok(equips) = equip_info::get_by_use_char(pool, uid, idx).await {
            let equip_inven_index: Vec<i64> =
                equips.iter().filter_map(|e| e.inven_index).collect();
            char_equip_info.push(PresetUseEquipInfo {
                char_inven_index: Some(idx),
                equip_inven_index,
            });
        }
    }

    let response = MonsterHuntPresetUseResponse {
        monster_hunt_deck_info,
        char_info,
        char_equip_info,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MonsterHuntPresetUse.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
