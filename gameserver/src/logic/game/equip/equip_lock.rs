use bd2::prost::Message;
use bd2::proto::proto_net::{EquipLockRequest, EquipLockResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipLockRequest) -> GameResponse {
    info!("Handling EquipLockRequest: {:?}", req);

    if let Some(inven_index) = req.inven_index {
        let lock_flag = req.lock_flag.unwrap_or(1);
        let _ =
            database::db::equip::equip_info::set_lock_flag(pool, uid, inven_index, lock_flag).await;
    }

    let response = EquipLockResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipLock.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
