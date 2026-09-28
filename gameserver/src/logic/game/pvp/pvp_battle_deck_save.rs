use bd2::prost::Message;
use bd2::proto::proto_net::{PvpBattleDeckSaveRequest, PvpBattleDeckSaveResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pvp::pvp_deck_info::{self, DeckSlot};
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, items_to_json};

pub async fn handle(pool: &SqlitePool, uid: i64, req: PvpBattleDeckSaveRequest) -> GameResponse {
    info!("Handling PvpBattleDeckSaveRequest: {:?}", req);

    let deck_type = req.deck_type.unwrap_or(0);
    let slots: Vec<DeckSlot> = req
        .deck_info
        .iter()
        .enumerate()
        .map(|(i, d)| DeckSlot {
            position: d.position.unwrap_or(i as i32),
            char_inven_index: d.char_inven_index.unwrap_or(0),
            sequence: d.sequence,
            costume_inven_index: d.costume_inven_index,
        })
        .collect();
    let _ = pvp_deck_info::replace_all(pool, uid, deck_type, &slots).await;

    let json = items_to_json(&req.contents_item_info);
    let _ = pvp_deck_info::set_meta(pool, uid, deck_type, &json, req.battle_power.unwrap_or(0)).await;

    let response = PvpBattleDeckSaveResponse {};

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PvpBattleDeckSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
