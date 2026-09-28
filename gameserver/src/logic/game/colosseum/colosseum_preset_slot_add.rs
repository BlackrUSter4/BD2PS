use bd2::prost::Message;
use bd2::proto::proto_net::{ColosseumPresetSlotAddRequest, ColosseumPresetSlotAddResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::colosseum::colosseum_preset_info as preset_db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumPresetSlotAddRequest) -> GameResponse {
    info!("Handling ColosseumPresetSlotAddRequest: {:?}", req);

    let add_count = req.add_count.unwrap_or(1).max(0);
    let mut next_slot = preset_db::max_slot(pool, uid).await.unwrap_or(-1) + 1;
    for _ in 0..add_count {
        let _ = preset_db::insert(pool, uid, next_slot, None, None, None).await;
        next_slot += 1;
    }

    let response = ColosseumPresetSlotAddResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumPresetSlotAdd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
