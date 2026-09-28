use super::{encode_index_list, parse_index_list};
use bd2::prost::Message;
use bd2::proto::proto_net::{EquipPresetSaveRequest, EquipPresetSaveResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::models::game::equip::{
    equip_preset_char_info::EquipPresetCharInfo, equip_preset_info::EquipPresetInfo,
    equip_preset_item_info::EquipPresetItemInfo,
};
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipPresetSaveRequest) -> GameResponse {
    info!("Handling EquipPresetSaveRequest: {:?}", req);

    if let Some(char_inven_index) = req.char_inven_index {
        let mut item_indices = vec![];
        for item in &req.equip_info {
            if let Ok(idx) = database::db::equip::equip_preset_item_info::insert(
                pool,
                &EquipPresetItemInfo {
                    index: 0,
                    uid,
                    equip_type: item.equip_type,
                    equip_inven_index: item.equip_inven_index,
                },
            )
            .await
            {
                item_indices.push(idx);
            }
        }

        if let Ok(preset_index) = database::db::equip::equip_preset_info::insert(
            pool,
            &EquipPresetInfo {
                index: 0,
                uid,
                preset_name: req.preset_name.clone(),
                slot: req.slot,
                preset_resource_id: req.preset_resource_id,
                preset_resource_color: req.preset_resource_color,
                item_info_index: encode_index_list(&item_indices),
            },
        )
        .await
        {
            let char_rows =
                database::db::equip::equip_preset_char_info::get_equip_preset_char_info(pool, uid)
                    .await
                    .unwrap_or_default();
            if let Some(existing) = char_rows
                .iter()
                .find(|c| c.char_inven_index == Some(char_inven_index))
            {
                let mut presets = parse_index_list(&existing.preset_info_index);
                presets.push(preset_index);
                let _ = database::db::equip::equip_preset_char_info::add_equip_preset_char_info(
                    pool,
                    &EquipPresetCharInfo {
                        index: existing.index,
                        uid,
                        char_inven_index: Some(char_inven_index),
                        preset_info_index: encode_index_list(&presets),
                    },
                )
                .await;
                // Replace the old row's list rather than duplicate it.
                let _ = sqlx::query(
                    "DELETE FROM EquipPresetCharInfo WHERE Uid = ? AND \"Index\" = ?",
                )
                .bind(uid)
                .bind(existing.index)
                .execute(pool)
                .await;
            } else {
                let _ = database::db::equip::equip_preset_char_info::insert(
                    pool,
                    &EquipPresetCharInfo {
                        index: 0,
                        uid,
                        char_inven_index: Some(char_inven_index),
                        preset_info_index: encode_index_list(&[preset_index]),
                    },
                )
                .await;
            }
        }
    }

    let response = EquipPresetSaveResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipPresetSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
