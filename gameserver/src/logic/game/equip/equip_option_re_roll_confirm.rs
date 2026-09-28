use super::{get_equip_with_base, to_dbinfo};
use bd2::prost::Message;
use bd2::proto::proto_net::{EquipOptionReRollConfirmRequest, EquipOptionReRollConfirmResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipOptionReRollConfirmRequest) -> GameResponse {
    info!("Handling EquipOptionReRollConfirmRequest: {:?}", req);

    let mut equip_info = None;
    if let Some(inven_index) = req.equip_inven_index {
        if let Some((_equip, Some(base))) = get_equip_with_base(pool, uid, inven_index).await {
            if req.is_confirm.unwrap_or(true) {
                let _ = database::db::equip::equip_base_info::clear_reroll_stash(pool, uid, base.index).await;
            } else {
                let _ = database::db::equip::equip_base_info::revert_reroll(pool, uid, base.index).await;
            }
        }
        if let Some((equip, base)) = get_equip_with_base(pool, uid, inven_index).await {
            equip_info = Some(to_dbinfo(&equip, base.as_ref()));
        }
    }

    let response = EquipOptionReRollConfirmResponse { equip_info, char_info: None };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipOptionReRollConfirm.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
