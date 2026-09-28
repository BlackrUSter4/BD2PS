use bd2::prost::Message;
use bd2::proto::proto_net::{IbDeckSaveRequest, IbDeckSaveResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::ib::ib_deck::{self, DeckSlot};
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(pool: &SqlitePool, uid: i64, req: IbDeckSaveRequest) -> GameResponse {
    info!("Handling IbDeckSaveRequest: {:?}", req);

    let slots: Vec<DeckSlot> = req
        .deck_info
        .iter()
        .enumerate()
        .map(|(i, d)| DeckSlot {
            position: d.position.unwrap_or(i as i32),
            inven_index: d.inven_index.unwrap_or(0),
            rotation_count: d.rotation_count.unwrap_or(0),
        })
        .collect();
    let _ = ib_deck::replace_all(pool, uid, &slots).await;

    let response = IbDeckSaveResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::IbDeckSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
