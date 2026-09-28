use bd2::prost::Message;
use bd2::proto::proto_net::{PvpBattleDeckInfoRequest, PvpBattleDeckInfoResponse, PvpBattleUserDeckInfo as DeckInfoProto};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pvp::pvp_deck_info;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, json_to_items};

async fn deck(pool: &SqlitePool, uid: i64, deck_type: i32) -> Vec<DeckInfoProto> {
    pvp_deck_info::get_by_uid_type(pool, uid, deck_type)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|d| DeckInfoProto {
            char_inven_index: Some(d.char_inven_index),
            position: Some(d.position),
            sequence: d.sequence,
            costume_inven_index: d.costume_inven_index,
            costume_inven_index_seq: vec![],
            priority_skill_costume_inven_index: vec![],
        })
        .collect()
}

async fn items(pool: &SqlitePool, uid: i64, deck_type: i32) -> Vec<bd2::proto::proto_net::ContentsCharItemInfo> {
    pvp_deck_info::get_meta(pool, uid, deck_type)
        .await
        .ok()
        .flatten()
        .map(|m| json_to_items(&m.item_info_json))
        .unwrap_or_default()
}

pub async fn handle(pool: &SqlitePool, uid: i64, req: PvpBattleDeckInfoRequest) -> GameResponse {
    info!("Handling PvpBattleDeckInfoRequest: {:?}", req);

    let response = PvpBattleDeckInfoResponse {
        attack_deck_info: deck(pool, uid, 0).await,
        attack_deck_item_info: items(pool, uid, 0).await,
        defense_deck_info: deck(pool, uid, 1).await,
        defense_deck_item_info: items(pool, uid, 1).await,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PvpBattleDeckInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
