use bd2::prost::Message;
use bd2::proto::proto_net::{ColosseumDeckSaveRequest, ColosseumDeckSaveResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, save_deck};

pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumDeckSaveRequest) -> GameResponse {
    info!("Handling ColosseumDeckSaveRequest: {:?}", req);

    let _ = save_deck(pool, uid, &req.deck_info, &req.contents_item_info).await;

    let response = ColosseumDeckSaveResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumDeckSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
