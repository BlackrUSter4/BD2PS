use bd2::prost::Message;
use bd2::proto::proto_net::{TacticsBingoDeckSaveRequest, TacticsBingoDeckSaveResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::tactics_bingo::tactics_bingo_deck_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::DeckEntry;

pub async fn handle(pool: &SqlitePool, uid: i64, req: TacticsBingoDeckSaveRequest) -> GameResponse {
    info!("Handling TacticsBingoDeckSaveRequest: {:?}", req);

    let entries: Vec<DeckEntry> = req
        .deck_info
        .iter()
        .map(|d| DeckEntry {
            char_inven_index: d.char_inven_index,
            char_id: d.char_id,
            costume_id: d.costume_id,
            position: d.position,
            sequence: d.sequence,
        })
        .collect();
    let json = serde_json::to_string(&entries).ok();
    let _ = db::save(pool, uid, json.as_deref()).await;

    let response = TacticsBingoDeckSaveResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::TacticsBingoDeckSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
