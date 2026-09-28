use bd2::prost::Message;
use bd2::proto::proto_net::{EquipAddSlotRequest, EquipAddSlotResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// No slot-capacity column/table exists anywhere for equip inventory (unlike Life/Fishing,
/// which do track this) — accepted unconditionally, same "nothing to validate against" call
/// made in earlier rounds for similar capacity-expansion requests with no backing table.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: EquipAddSlotRequest) -> GameResponse {
    info!("Handling EquipAddSlotRequest: {:?}", req);

    let response = EquipAddSlotResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipAddSlot.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
