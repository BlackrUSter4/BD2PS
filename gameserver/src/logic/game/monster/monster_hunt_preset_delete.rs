use bd2::prost::Message;
use bd2::proto::proto_net::{MonsterHuntPresetDeleteRequest, MonsterHuntPresetDeleteResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::monster::monster_hunt_preset_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: MonsterHuntPresetDeleteRequest,
) -> GameResponse {
    info!("Handling MonsterHuntPresetDeleteRequest: {:?}", req);

    let _ = db::delete_slots(pool, uid, &req.slot).await;

    let response = MonsterHuntPresetDeleteResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MonsterHuntPresetDelete.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
