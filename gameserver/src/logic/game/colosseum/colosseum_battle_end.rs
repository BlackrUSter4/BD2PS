use bd2::prost::Message;
use bd2::proto::proto_net::{ColosseumBattleEndRequest, ColosseumBattleEndResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::colosseum::{colosseum_battle_history, colosseum_match_candidate, colosseum_user_info};
use sqlx::SqlitePool;
use tracing::info;

use super::{
    default_notify, empty_reward_bundle, grant_new_promotion_rewards, now_ms, CHANGE_VP_LOSE,
    CHANGE_VP_WIN, CURRENT_SEASON,
};

pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumBattleEndRequest) -> GameResponse {
    info!("Handling ColosseumBattleEndRequest: {:?}", req);

    let user = colosseum_user_info::get_or_create(pool, uid).await.unwrap_or_default();
    let won = req.battle_result.unwrap_or(0) == super::BATTLE_RESULT_WIN;
    let vp_delta = if won { CHANGE_VP_WIN } else { CHANGE_VP_LOSE };
    let now = now_ms();

    let enemy_owner_index = user.current_battle_enemy_index;
    let candidate = match enemy_owner_index {
        Some(idx) => colosseum_match_candidate::find(pool, uid, idx).await.ok().flatten(),
        None => None,
    };

    let new_vp = colosseum_user_info::apply_battle_result(pool, uid, vp_delta, won).await.unwrap_or(user.vp);
    grant_new_promotion_rewards(pool, uid, new_vp).await;

    let _ = colosseum_battle_history::insert(
        pool,
        uid,
        CURRENT_SEASON,
        true, // this side is always the attacker in the Start/End flow
        enemy_owner_index,
        candidate.as_ref().and_then(|c| c.enemy_user_id.clone()).as_deref(),
        candidate.as_ref().and_then(|c| c.enemy_vp),
        None,
        Some(vp_delta),
        now,
        None,
        false,
    )
    .await;

    // Mirror a defense-side record for a real opponent (bots have no account to record against).
    if let Some(c) = &candidate {
        if !c.enemy_is_bot {
            let opp_won = !won;
            let opp_vp_delta = if opp_won { CHANGE_VP_WIN } else { CHANGE_VP_LOSE };
            let opp_new_vp =
                colosseum_user_info::apply_battle_result(pool, c.enemy_owner_index, opp_vp_delta, opp_won)
                    .await
                    .unwrap_or(0);
            grant_new_promotion_rewards(pool, c.enemy_owner_index, opp_new_vp).await;

            let account = database::db::user::user::find_account(pool, uid).await.ok().flatten();
            let _ = colosseum_battle_history::insert(
                pool,
                c.enemy_owner_index,
                CURRENT_SEASON,
                false,
                Some(uid),
                account.map(|a| a.user_name).as_deref(),
                Some(new_vp),
                None,
                Some(opp_vp_delta),
                now,
                None,
                false,
            )
            .await;
        }
    }

    let _ = colosseum_user_info::set_current_battle_enemy(pool, uid, None).await;
    let rank = colosseum_user_info::rank_of(pool, uid).await.unwrap_or(1);

    let response = ColosseumBattleEndResponse {
        battle_result: req.battle_result,
        change_vp: Some(vp_delta),
        // No win-streak bonus formula exists in captured data — flagged placeholder.
        continue_win_vp: Some(0),
        battle_statistics_info: None,
        reward_info_bundle: Some(empty_reward_bundle()),
        result_top_percent: Some(100.0 * rank as f64 / (user.win_count + user.lose_count + 1).max(rank) as f64),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumBattleEnd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
