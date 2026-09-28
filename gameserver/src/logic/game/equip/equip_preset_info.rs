use super::{parse_index_list};
use bd2::prost::Message;
use bd2::proto::proto_net::{
    EquipPresetCharInfo as ProtoPresetChar, EquipPresetDbInfo, EquipPresetInfoRequest,
    EquipPresetInfoResponse, EquipPresetItemInfo as ProtoPresetItem, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::models::game::equip::equip_preset_info::EquipPresetInfo;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipPresetInfoRequest) -> GameResponse {
    info!("Handling EquipPresetInfoRequest: {:?}", req);

    let char_rows = database::db::equip::equip_preset_char_info::get_equip_preset_char_info(
        pool, uid,
    )
    .await
    .unwrap_or_default();
    let all_presets = database::db::equip::equip_preset_info::get_equip_preset_info(pool, uid)
        .await
        .unwrap_or_default();
    let all_items = database::db::equip::equip_preset_item_info::get_equip_preset_item_info(
        pool, uid,
    )
    .await
    .unwrap_or_default();

    let build_preset = |p: &EquipPresetInfo| -> EquipPresetDbInfo {
        let item_indices = parse_index_list(&p.item_info_index);
        let item_info = all_items
            .iter()
            .filter(|it| item_indices.contains(&it.index))
            .map(|it| ProtoPresetItem {
                equip_type: it.equip_type,
                equip_inven_index: it.equip_inven_index,
            })
            .collect();
        EquipPresetDbInfo {
            preset_name: p.preset_name.clone(),
            slot: p.slot,
            preset_resource_id: p.preset_resource_id,
            preset_resource_color: p.preset_resource_color,
            item_info,
        }
    };

    let char_info = char_rows
        .iter()
        .map(|c| {
            let preset_indices = parse_index_list(&c.preset_info_index);
            let preset_info = all_presets
                .iter()
                .filter(|p| preset_indices.contains(&p.index))
                .map(build_preset)
                .collect();
            ProtoPresetChar {
                char_inven_index: c.char_inven_index,
                preset_info,
            }
        })
        .collect();

    let response = EquipPresetInfoResponse { char_info };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipPresetInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
