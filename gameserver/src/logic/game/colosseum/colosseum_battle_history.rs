use bd2::prost::Message;
use bd2::proto::proto_net::{
    ColosseumBattleHistoryInfo, ColosseumBattleHistoryRequest, ColosseumBattleHistoryResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::colosseum::colosseum_battle_history;
use database::models::game::colosseum::colosseum_battle_history::ColosseumBattleHistory;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, BATTLE_HISTORY_LIMIT, CURRENT_SEASON};

fn to_proto(row: ColosseumBattleHistory) -> ColosseumBattleHistoryInfo {
    ColosseumBattleHistoryInfo {
        battle_index: Some(row.index),
        battle_result: Some(if row.change_vp.unwrap_or(0) > 0 { 1 } else { 0 }),
        enemy_owner_index: row.enemy_owner_index,
        enemy_user_id: row.enemy_user_id,
        enemy_vp: row.enemy_vp,
        enemy_rank: row.enemy_rank,
        enemy_costume_id: vec![],
        enemy_costume_design_id: vec![],
        change_vp: row.change_vp,
        time_value: row.time_value,
        enemy_top_percent: row.enemy_top_percent,
        enemy_bless_id_list: vec![],
        is_no_game: Some(row.is_no_game),
    }
}

pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumBattleHistoryRequest) -> GameResponse {
    info!("Handling ColosseumBattleHistoryRequest: {:?}", req);

    let attack_rows = colosseum_battle_history::recent_by_uid(pool, uid, true, BATTLE_HISTORY_LIMIT)
        .await
        .unwrap_or_default();
    let defense_rows = colosseum_battle_history::recent_by_uid(pool, uid, false, BATTLE_HISTORY_LIMIT)
        .await
        .unwrap_or_default();

    let (season_attack_win, season_attack_lose) =
        colosseum_battle_history::win_lose_count(pool, uid, CURRENT_SEASON, true).await.unwrap_or((0, 0));
    let (season_defense_win, season_defense_lose) =
        colosseum_battle_history::win_lose_count(pool, uid, CURRENT_SEASON, false).await.unwrap_or((0, 0));
    let (prev_attack_win, prev_attack_lose) =
        colosseum_battle_history::win_lose_count(pool, uid, CURRENT_SEASON - 1, true).await.unwrap_or((0, 0));
    let (prev_defense_win, prev_defense_lose) =
        colosseum_battle_history::win_lose_count(pool, uid, CURRENT_SEASON - 1, false).await.unwrap_or((0, 0));

    let response = ColosseumBattleHistoryResponse {
        attack_history_info: attack_rows.into_iter().map(to_proto).collect(),
        defense_history_info: defense_rows.into_iter().map(to_proto).collect(),
        season_win_count: Some(season_attack_win + season_defense_win),
        season_lose_count: Some(season_attack_lose + season_defense_lose),
        season_attack_win_count: Some(season_attack_win),
        season_attack_lose_count: Some(season_attack_lose),
        season_defense_win_count: Some(season_defense_win),
        season_defense_lose_count: Some(season_defense_lose),
        prev_season_attack_win_count: Some(prev_attack_win),
        prev_season_attack_lose_count: Some(prev_attack_lose),
        prev_season_defense_win_count: Some(prev_defense_win),
        prev_season_defense_lose_count: Some(prev_defense_lose),
        // This project doesn't track per-deck-loadout stats separately from overall season
        // stats (no evidence a deck can be swapped mid-season in a way that would need it
        // distinguished) — reusing the same season totals rather than fabricating a split.
        deck_season_attack_win_count: Some(season_attack_win),
        deck_season_attack_lose_count: Some(season_attack_lose),
        deck_season_defense_win_count: Some(season_defense_win),
        deck_season_defense_lose_count: Some(season_defense_lose),
        deck_season_attack_reset_date: None,
        deck_season_defense_reset_date: None,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumBattleHistory.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
