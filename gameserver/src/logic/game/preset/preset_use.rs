use bd2::prost::Message;
use bd2::proto::proto_net::{
    CharDbInfo, DeckDbInfo, PresetUseEquipInfo as PresetUseEquipInfoProto, PresetUseRequest,
    PresetUseResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{
    char::char_info, deck::deck_info as deck_db, preset::preset_info as preset_db,
    preset::preset_use_equip_info as use_equip_db,
};
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real "use preset" — applies the saved preset's deck into the account's actual active
/// DeckInfo (same table DeckSave/DeckInfo read from), records the applied equip snapshot in
/// PresetUseEquipInfo, and returns the real resulting deck + real character rows.
pub async fn handle(pool: &SqlitePool, uid: i64, req: PresetUseRequest) -> GameResponse {
    info!("Handling PresetUseRequest: {:?}", req);

    let slot = req.slot.unwrap_or(0);

    let mut deck_info = Vec::new();
    let mut char_info = Vec::new();
    let mut char_equip_info = Vec::new();

    if let Some(preset) = preset_db::get_by_uid_and_slot(pool, uid, slot).await.ok().flatten() {
        let deck_rows = preset_db::get_deck(pool, preset.index).await.unwrap_or_default();

        let entries: Vec<(i64, Option<i32>, Option<i32>)> = deck_rows
            .iter()
            .map(|row| (row.char_inven_index, row.position, row.sequence))
            .collect();
        let _ = deck_db::replace_deck_info(pool, uid, &entries).await;

        let mut chars_equips = Vec::with_capacity(deck_rows.len());
        for row in &deck_rows {
            deck_info.push(DeckDbInfo {
                char_inven_index: Some(row.char_inven_index),
                position: row.position,
                sequence: row.sequence,
            });

            if let Ok(Some(char_row)) = char_info::get_by_inven_index(pool, uid, row.char_inven_index).await {
                char_info.push(CharDbInfo {
                    inven_index: char_row.inven_index,
                    id: char_row.id,
                    hp: char_row.hp,
                    level: char_row.level,
                    costume_id: char_row.costume_id,
                    exp: char_row.exp,
                    use_costume: char_row.use_costume,
                    talent_level: char_row.talent_level,
                    talent_exp: char_row.talent_exp,
                    solidarity_reward: char_row.solidarity_reward,
                    expiry_time: char_row.expiry_time,
                    pictorialbook_info: vec![],
                    connect_potential_costume: char_row.connect_potential_costume,
                });
            }

            let equips = preset_db::get_deck_equips(pool, row.index).await.unwrap_or_default();
            let equip_inven_index: Vec<i64> = equips.iter().filter_map(|e| e.equip_inven_index).collect();
            char_equip_info.push(PresetUseEquipInfoProto {
                char_inven_index: Some(row.char_inven_index),
                equip_inven_index: equip_inven_index.clone(),
            });
            chars_equips.push((row.char_inven_index, equip_inven_index));
        }

        let _ = use_equip_db::replace_for_uid(pool, uid, &chars_equips).await;
    }

    let response = PresetUseResponse { deck_info, char_info, char_equip_info };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PresetUse.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
