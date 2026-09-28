use bd2::prost::Message;
use bd2::proto::proto_net::{PvpBattleEndRequest, PvpBattleEndResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pvp::{pvp_battle_history, pvp_current_match, pvp_user_info};
use sqlx::SqlitePool;
use tracing::info;

use super::{build_deck_snapshot, default_notify, grant_battle_reward, grant_new_once_rewards, lose_point_for_vp, now_ms, win_point_for_vp, BATTLE_RESULT_WIN};

pub async fn handle(pool: &SqlitePool, uid: i64, req: PvpBattleEndRequest) -> GameResponse {
    info!("Handling PvpBattleEndRequest: {:?}", req);

    let user = pvp_user_info::get_or_create(pool, uid).await.unwrap_or_default();
    let won = req.battle_result.unwrap_or(0) == BATTLE_RESULT_WIN;
    let vp_delta = if won { win_point_for_vp(user.vp) } else { lose_point_for_vp(user.vp) };
    let now = now_ms();
    let season = super::current_season();

    let match_info = pvp_current_match::get(pool, uid).await.ok().flatten();
    let deck_snapshot = build_deck_snapshot(pool, uid, &match_info).await;

    let new_vp = pvp_user_info::apply_attack_result(pool, uid, vp_delta, won).await.unwrap_or(user.vp);
    let item_info = grant_battle_reward(pool, uid, new_vp, won).await;
    let once_reward_info = grant_new_once_rewards(pool, uid, user.vp, new_vp).await;

    let _ = pvp_battle_history::insert(
        pool,
        uid,
        season,
        true,
        match_info.as_ref().and_then(|m| m.enemy_owner_index),
        match_info.as_ref().and_then(|m| m.enemy_user_id.clone()).as_deref(),
        match_info.as_ref().and_then(|m| m.enemy_vp),
        None,
        Some(vp_delta),
        Some(0),
        now,
        false,
        match_info.as_ref().and_then(|m| m.battle_random_seed),
        Some(&deck_snapshot),
    )
    .await;

    // Mirror a defense-side record for a real opponent (bots have no account to record against).
    if let Some(m) = &match_info {
        if !m.enemy_is_bot {
            if let Some(enemy_uid) = m.enemy_owner_index {
                let opp_won = !won;
                let opp_delta = if opp_won { win_point_for_vp(m.enemy_vp.unwrap_or(1000)) } else { lose_point_for_vp(m.enemy_vp.unwrap_or(1000)) };
                let opp_new_vp = pvp_user_info::apply_defense_result(pool, enemy_uid, opp_delta, opp_won)
                    .await
                    .unwrap_or(0);
                let _ = grant_battle_reward(pool, enemy_uid, opp_new_vp, opp_won).await;
                let account = database::db::user::user::find_account(pool, uid).await.ok().flatten();
                let _ = pvp_battle_history::insert(
                    pool,
                    enemy_uid,
                    season,
                    false,
                    Some(uid),
                    account.map(|a| a.user_name).as_deref(),
                    Some(new_vp),
                    None,
                    Some(opp_delta),
                    Some(0),
                    now,
                    false,
                    None,
                    None,
                )
                .await;
            }
        }
    }

    let _ = pvp_user_info::set_current_battle_enemy(pool, uid, None).await;
    let _ = pvp_current_match::clear(pool, uid).await;

    let response = PvpBattleEndResponse {
        battle_result: req.battle_result,
        item_info,
        char_partner_info: vec![],
        change_vp: Some(vp_delta),
        // No win-streak bonus formula exists anywhere in captured data — documented placeholder.
        continue_win_vp: Some(0),
        battle_statistics_info: None,
        once_reward_info,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PvpBattleEnd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
