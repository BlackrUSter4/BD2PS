use super::{encode_option_list, get_equip_with_base};
use bd2::prost::Message;
use bd2::proto::proto_net::{EquipMainOptChangeRequest, EquipMainOptChangeResponse, EquipOptionInfo, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// The request carries a single (group_id, id) pair with no slot index, so it replaces the
/// equip's whole main-option list with that one entry — the most direct reading the schema
/// supports; no table confirms whether multi-main-option items should keep other slots.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipMainOptChangeRequest) -> GameResponse {
    info!("Handling EquipMainOptChangeRequest: {:?}", req);

    if let Some(inven_index) = req.inven_index {
        if let (Some(group_id), Some(id)) = (req.group_id, req.id) {
            if let Some((_equip, Some(base))) = get_equip_with_base(pool, uid, inven_index).await {
                let new_main = vec![EquipOptionInfo {
                    group_id: Some(group_id),
                    id: Some(id),
                }];
                let _ = database::db::equip::equip_base_info::set_options(
                    pool,
                    uid,
                    base.index,
                    encode_option_list(&new_main).as_deref(),
                    base.sub_option_index.as_deref(),
                )
                .await;
            }
        }
    }

    let response = EquipMainOptChangeResponse { char_info: None };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipMainOptChange.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
