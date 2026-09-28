use bd2::prost::Message;
use bd2::proto::proto_net::{PresetDeleteRequest, PresetDeleteResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::preset::preset_info as preset_db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(pool: &SqlitePool, uid: i64, req: PresetDeleteRequest) -> GameResponse {
    info!("Handling PresetDeleteRequest: {:?}", req);

    let _ = preset_db::delete_by_slots(pool, uid, &req.slot).await;

    let response = PresetDeleteResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PresetDelete.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
