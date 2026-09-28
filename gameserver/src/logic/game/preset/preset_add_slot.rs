use bd2::prost::Message;
use bd2::proto::proto_net::{PresetAddSlotRequest, PresetAddSlotResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::preset::preset_info as preset_db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(pool: &SqlitePool, uid: i64, req: PresetAddSlotRequest) -> GameResponse {
    info!("Handling PresetAddSlotRequest: {:?}", req);

    let add_count = req.add_count.unwrap_or(1).max(0);
    let mut next_slot = preset_db::max_slot(pool, uid).await.unwrap_or(-1) + 1;
    for _ in 0..add_count {
        let _ = preset_db::insert(pool, uid, next_slot, None, None, None).await;
        next_slot += 1;
    }

    let response = PresetAddSlotResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PresetAddSlot.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
