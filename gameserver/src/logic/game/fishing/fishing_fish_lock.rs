use bd2::prost::Message;
use bd2::proto::proto_net::{FishingFishLockRequest, FishingFishLockResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingFishLockRequest) -> GameResponse {
    info!("Handling FishingFishLockRequest: {:?}", req);

    for entry in &req.fish_lock_info {
        if let Some(inven_index) = entry.inven_index {
            let is_lock = entry.is_lock.unwrap_or(false);
            let _ = database::db::fishing::fishing_fish_info::set_lock(pool, uid, inven_index, is_lock).await;
        }
    }

    let response = FishingFishLockResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::FishingFishLock.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
