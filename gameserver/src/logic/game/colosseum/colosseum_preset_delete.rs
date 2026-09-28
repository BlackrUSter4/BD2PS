use bd2::prost::Message;
use bd2::proto::proto_net::{ColosseumPresetDeleteRequest, ColosseumPresetDeleteResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::colosseum::colosseum_preset_info as preset_db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumPresetDeleteRequest) -> GameResponse {
    info!("Handling ColosseumPresetDeleteRequest: {:?}", req);

    let _ = preset_db::delete_by_slots(pool, uid, &req.slot).await;

    let response = ColosseumPresetDeleteResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumPresetDelete.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
