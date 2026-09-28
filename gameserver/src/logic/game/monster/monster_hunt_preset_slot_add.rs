use bd2::prost::Message;
use bd2::proto::proto_net::{MonsterHuntPresetSlotAddRequest, MonsterHuntPresetSlotAddResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::monster::monster_hunt_preset_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: MonsterHuntPresetSlotAddRequest,
) -> GameResponse {
    info!("Handling MonsterHuntPresetSlotAddRequest: {:?}", req);

    let add_count = req.add_count.unwrap_or(1).max(0);
    let mut next = db::max_slot(pool, uid).await.unwrap_or(0);
    for _ in 0..add_count {
        next += 1;
        let _ = db::add_slot(pool, uid, next).await;
    }

    let response = MonsterHuntPresetSlotAddResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MonsterHuntPresetSlotAdd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
