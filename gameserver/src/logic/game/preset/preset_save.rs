use bd2::prost::Message;
use bd2::proto::proto_net::{PresetSaveRequest, PresetSaveResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::preset::preset_info as preset_db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(pool: &SqlitePool, uid: i64, req: PresetSaveRequest) -> GameResponse {
    info!("Handling PresetSaveRequest: {:?}", req);

    if let Some(preset) = &req.preset_info {
        let slot = preset.slot.unwrap_or(0);
        let existing = preset_db::get_by_uid_and_slot(pool, uid, slot).await.ok().flatten();
        let preset_index = match existing {
            Some(row) => {
                let _ = preset_db::update_info(
                    pool,
                    uid,
                    slot,
                    preset.preset_name.as_deref(),
                    preset.preset_resource_id,
                    preset.preset_resource_color,
                )
                .await;
                row.index
            }
            None => preset_db::insert(
                pool,
                uid,
                slot,
                preset.preset_name.as_deref(),
                preset.preset_resource_id,
                preset.preset_resource_color,
            )
            .await
            .unwrap_or(0),
        };

        let slots: Vec<preset_db::PresetDeckSlot> = preset
            .deck_info
            .iter()
            .filter_map(|d| {
                let char_inven_index = d.deck_base_info.as_ref().and_then(|b| b.char_inven_index)?;
                Some(preset_db::PresetDeckSlot {
                    char_inven_index,
                    position: d.deck_base_info.as_ref().and_then(|b| b.position),
                    sequence: d.deck_base_info.as_ref().and_then(|b| b.sequence),
                    costume_inven_index: d.costume_inven_index,
                    team: d.team,
                    equips: d.equip_info.iter().map(|e| (e.equip_type, e.equip_inven_index)).collect(),
                })
            })
            .collect();
        let _ = preset_db::replace_deck(pool, preset_index, &slots).await;
    }

    let response = PresetSaveResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PresetSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
