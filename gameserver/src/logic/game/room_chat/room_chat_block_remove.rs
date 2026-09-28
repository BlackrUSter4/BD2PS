use bd2::prost::Message;
use bd2::proto::proto_net::{RoomChatBlockRemoveRequest, RoomChatBlockRemoveResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::room_chat::room_chat_block;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: RoomChatBlockRemoveRequest) -> GameResponse {
    info!("Handling RoomChatBlockRemoveRequest: {:?}", req);

    let target_owner_index = req.target_owner_index.unwrap_or_default();
    room_chat_block::remove(pool, uid, target_owner_index).await;

    let response = RoomChatBlockRemoveResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::RoomChatBlockRemove.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
