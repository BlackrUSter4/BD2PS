use bd2::prost::Message;
use bd2::proto::proto_net::{PvpBattleRankUserDetailRequest, PvpBattleRankUserDetailResponse, PvpBattleUserDeckFullInfo, PvpBattleUserDeckInfo as DeckInfoProto};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pvp::{pvp_deck_info, pvp_user_info};
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

async fn deck_full(pool: &SqlitePool, uid: i64, deck_type: i32) -> PvpBattleUserDeckFullInfo {
    let slots = pvp_deck_info::get_by_uid_type(pool, uid, deck_type).await.unwrap_or_default();
    PvpBattleUserDeckFullInfo {
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
    }
}

/// Real cross-account read: another account's win/loss record and both real saved decks.
pub async fn handle(pool: &SqlitePool, _uid: i64, req: PvpBattleRankUserDetailRequest) -> GameResponse {
    info!("Handling PvpBattleRankUserDetailRequest: {:?}", req);

    let target = req.target_owner_index.unwrap_or(0);
    let user = pvp_user_info::get(pool, target).await.ok().flatten().unwrap_or_default();

    let response = PvpBattleRankUserDetailResponse {
        owner_index: Some(target),
        win_count: Some(user.win_count),
        lose_count: Some(user.lose_count),
        attack_deck_info: Some(deck_full(pool, target, 0).await),
        defence_deck_info: Some(deck_full(pool, target, 1).await),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PvpBattleRankUserDetail.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
