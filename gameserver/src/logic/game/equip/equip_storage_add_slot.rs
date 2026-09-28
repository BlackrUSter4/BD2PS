use bd2::prost::Message;
use bd2::proto::proto_net::{EquipStorageAddSlotRequest, EquipStorageAddSlotResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// No slot-capacity table exists — accepted unconditionally, same as EquipAddSlot.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: EquipStorageAddSlotRequest) -> GameResponse {
    info!("Handling EquipStorageAddSlotRequest: {:?}", req);

    let response = EquipStorageAddSlotResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipStorageAddSlot.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
