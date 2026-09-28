use bd2::prost::Message;
use bd2::proto::proto_net::{RoomChatBlockAddRequest, RoomChatBlockAddResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::room_chat::room_chat_block;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: RoomChatBlockAddRequest) -> GameResponse {
    info!("Handling RoomChatBlockAddRequest: {:?}", req);

    let target_owner_index = req.target_owner_index.unwrap_or_default();
    let target_user_id = room_chat_block::resolve_user_id(pool, target_owner_index).await;
    let now = chrono::Utc::now().timestamp_millis();

    room_chat_block::add(pool, uid, target_owner_index, target_user_id.as_deref(), now).await;

    let response = RoomChatBlockAddResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::RoomChatBlockAdd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
