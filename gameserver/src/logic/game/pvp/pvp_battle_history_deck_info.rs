use bd2::prost::Message;
use bd2::proto::proto_net::{PvpBattleHistoryDeckInfoRequest, PvpBattleHistoryDeckInfoResponse, PvpBattleUserDeckFullInfo, PvpBattleUserDeckInfo as DeckInfoProto};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pvp::pvp_battle_history;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Both decks are sourced from the real `DeckSnapshotJson` captured at battle-end time (a
/// {my: [...], enemy: [...]} pair of deck slots). No char/costume/equip/buff data is stored in
/// the snapshot (never fabricated), so those sub-lists are left empty — same convention as
/// every other async-battle system in this project (Colosseum/Ib/Guild raid).
pub async fn handle(pool: &SqlitePool, uid: i64, req: PvpBattleHistoryDeckInfoRequest) -> GameResponse {
    info!("Handling PvpBattleHistoryDeckInfoRequest: {:?}", req);

    let history = match req.battle_index {
        Some(idx) => pvp_battle_history::get_by_index(pool, uid, idx).await.ok().flatten(),
        None => None,
    };

    let (user_deck, enemy_deck) = history
        .and_then(|h| h.deck_snapshot_json)
        .and_then(|json| serde_json::from_str::<super::DeckSnapshot>(&json).ok())
        .map(|s| (s.my, s.enemy))
        .unwrap_or_default();

    let to_full = |slots: Vec<super::DeckSlotSnapshot>| PvpBattleUserDeckFullInfo {
        deck_info: slots
            .into_iter()
            .map(|d| DeckInfoProto {
                char_inven_index: Some(d.char_inven_index),
                position: Some(d.position),
                sequence: d.sequence,
                costume_inven_index: d.costume_inven_index,
                costume_inven_index_seq: vec![],
                priority_skill_costume_inven_index: vec![],
            })
            .collect(),
        char_info: vec![],
        costume_info: vec![],
        equip_info: vec![],
        buff_stat_info: vec![],
        awake_info: vec![],
    };

    let response = PvpBattleHistoryDeckInfoResponse {
        user_deck_full_info: Some(to_full(user_deck)),
        enemy_deck_full_info: Some(to_full(enemy_deck)),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PvpBattleHistoryDeckInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
