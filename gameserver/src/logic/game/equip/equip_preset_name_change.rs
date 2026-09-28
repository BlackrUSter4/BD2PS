use super::parse_index_list;
use bd2::prost::Message;
use bd2::proto::proto_net::{EquipPresetNameChangeRequest, EquipPresetNameChangeResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipPresetNameChangeRequest) -> GameResponse {
    info!("Handling EquipPresetNameChangeRequest: {:?}", req);

    if let Some(char_inven_index) = req.char_inven_index {
        let char_rows =
            database::db::equip::equip_preset_char_info::get_equip_preset_char_info(pool, uid)
                .await
                .unwrap_or_default();
        if let Some(char_row) = char_rows
            .iter()
            .find(|c| c.char_inven_index == Some(char_inven_index))
        {
            let preset_indices = parse_index_list(&char_row.preset_info_index);
            let presets = database::db::equip::equip_preset_info::get_equip_preset_info(pool, uid)
                .await
                .unwrap_or_default();
            if let Some(target) = presets
                .iter()
                .find(|p| preset_indices.contains(&p.index) && p.slot == req.slot)
            {
                let _ = database::db::equip::equip_preset_info::update(
                    pool,
                    uid,
                    target.index,
                    req.preset_name.as_deref(),
                    req.preset_resource_id,
                    req.preset_resource_color,
                    None,
                )
                .await;
            }
        }
    }

    let response = EquipPresetNameChangeResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipPresetNameChange.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
