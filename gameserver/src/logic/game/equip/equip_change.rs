use bd2::prost::Message;
use bd2::proto::proto_net::{EquipChangeRequest, EquipChangeResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Same (equip_index, char_index) field shape as EquipUseRequest, and no table distinguishes
/// "change" from "use" semantics — treated identically (re-point UseChar to the new equip).
pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipChangeRequest) -> GameResponse {
    info!("Handling EquipChangeRequest: {:?}", req);

    if let (Some(equip_index), Some(char_index)) = (req.equip_index, req.char_index) {
        let _ =
            database::db::equip::equip_info::set_use_char(pool, uid, equip_index, Some(char_index))
                .await;
    }

    let response = EquipChangeResponse { char_info: None };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipChange.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
