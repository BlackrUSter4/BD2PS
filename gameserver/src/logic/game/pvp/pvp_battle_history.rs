use bd2::prost::Message;
use bd2::proto::proto_net::{PvpBattleHistoryInfo, PvpBattleHistoryRequest, PvpBattleHistoryResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pvp::{pvp_battle_history, pvp_user_info};
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

fn to_proto(h: database::models::game::pvp::pvp_battle_history::PvpBattleHistory) -> PvpBattleHistoryInfo {
    // No dedicated result column is stored — derived from the real recorded Vp change, which is
    // itself always consistent with what actually happened (positive on a win, negative on a loss).
    let won = h.change_vp.unwrap_or(0) > 0;
    PvpBattleHistoryInfo {
        battle_index: Some(h.battle_index),
        battle_result: Some(if won { super::BATTLE_RESULT_WIN } else { 0 }),
        enemy_owner_index: h.enemy_owner_index,
        enemy_user_id: h.enemy_user_id,
        enemy_vp: h.enemy_vp,
        enemy_rank: h.enemy_rank,
        enemy_costume_id: vec![],
        enemy_costume_design_id: vec![],
        change_vp: h.change_vp,
        continue_win_vp: h.continue_win_vp,
        time_value: Some(h.time_value),
        is_no_game: Some(h.is_no_game),
    }
}

pub async fn handle(pool: &SqlitePool, uid: i64, req: PvpBattleHistoryRequest) -> GameResponse {
    info!("Handling PvpBattleHistoryRequest: {:?}", req);

    let user = pvp_user_info::get_or_create(pool, uid).await.unwrap_or_default();
    let limit = data::exceldb::get()
        .pvpdefaulttable
        .all()
        .first()
        .map(|d| d.battle_history_limit_count)
        .unwrap_or(20);

    let attack = pvp_battle_history::list_attack(pool, uid, limit).await.unwrap_or_default();
    let defense = pvp_battle_history::list_defense(pool, uid, limit).await.unwrap_or_default();

    let response = PvpBattleHistoryResponse {
        attack_history_info: attack.into_iter().map(to_proto).collect(),
        defense_history_info: defense.into_iter().map(to_proto).collect(),
        season_win_count: Some(user.season_attack_win_count + user.season_defense_win_count),
        season_lose_count: Some(user.season_attack_lose_count + user.season_defense_lose_count),
        season_attack_win_count: Some(user.season_attack_win_count),
        season_attack_lose_count: Some(user.season_attack_lose_count),
        season_defense_win_count: Some(user.season_defense_win_count),
        season_defense_lose_count: Some(user.season_defense_lose_count),
        prev_season_attack_win_count: Some(user.prev_season_attack_win_count),
        prev_season_attack_lose_count: Some(user.prev_season_attack_lose_count),
        prev_season_defense_win_count: Some(user.prev_season_defense_win_count),
        prev_season_defense_lose_count: Some(user.prev_season_defense_lose_count),
        deck_season_attack_win_count: Some(user.deck_season_attack_win_count),
        deck_season_attack_lose_count: Some(user.deck_season_attack_lose_count),
        deck_season_defense_win_count: Some(user.deck_season_defense_win_count),
        deck_season_defense_lose_count: Some(user.deck_season_defense_lose_count),
        deck_season_attack_reset_date: user.deck_season_attack_reset_time,
        deck_season_defense_reset_date: user.deck_season_defense_reset_time,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PvpBattleHistory.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
