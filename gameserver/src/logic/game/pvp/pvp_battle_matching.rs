use bd2::prost::Message;
use bd2::proto::proto_net::{PvpBattleMatchingRequest, PvpBattleMatchingResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pvp::{pvp_current_match, pvp_user_info};
use database::models::game::pvp::pvp_current_match::PvpCurrentMatch;
use rand::seq::IndexedRandom;
use rand::Rng;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, now_ms};

/// Single-opponent matching (unlike Colosseum's multi-candidate list — this request's response
/// shape carries one enemy_* set of fields, not a repeated list). Matches a real account when
/// one exists; otherwise synthesizes a bot from real `CharTable` ids near the caller's Vp, same
/// "flag it, don't fabricate" convention as Colosseum (no char/costume/equip/buff snapshot for
/// bots — no formula data exists for that here either).
pub async fn handle(pool: &SqlitePool, uid: i64, req: PvpBattleMatchingRequest) -> GameResponse {
    info!("Handling PvpBattleMatchingRequest: {:?}", req);

    let user = pvp_user_info::get_or_create(pool, uid).await.unwrap_or_default();
    let rank = pvp_user_info::rank_of(pool, uid).await.unwrap_or(1);
    let now = now_ms();

    let real_opponents = pvp_user_info::find_nearby_opponents(pool, uid, 1).await.unwrap_or_default();

    let (enemy_owner_index, enemy_user_id, enemy_vp, is_bot, char_ids, deck_info) =
        if let Some(opp) = real_opponents.into_iter().next() {
            let account = database::db::user::user::find_account(pool, opp.uid).await.ok().flatten();
            let name = account.map(|a| a.user_name).unwrap_or_else(|| format!("Player{}", opp.uid));
            let deck = database::db::pvp::pvp_deck_info::get_by_uid_type(pool, opp.uid, 1)
                .await
                .unwrap_or_default();
            (opp.uid, name, opp.vp, false, None, deck)
        } else {
            let all_chars = &data::exceldb::get().chartable;
            let mut rng = rand::thread_rng();
            let pool_ids: Vec<i32> = all_chars.all().iter().map(|c| c.id).collect();
            let bot_owner_index = -(now / 1000 + 1);
            let vp_jitter = rng.gen_range(-100..=100);
            let bot_vp = (user.vp + vp_jitter).max(0);
            let bot_chars: Vec<i32> = if pool_ids.is_empty() {
                vec![]
            } else {
                pool_ids.choose_multiple(&mut rng, 5.min(pool_ids.len())).copied().collect()
            };
            let bot_name = format!("Guest{}", rng.gen_range(1000..9999));
            let csv = bot_chars.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(",");
            (bot_owner_index, bot_name, bot_vp, true, Some(csv), vec![])
        };

    let seed = rand::thread_rng().gen_range(i32::MIN..=i32::MAX);

    let _ = pvp_current_match::set(
        pool,
        &PvpCurrentMatch {
            uid,
            enemy_owner_index: Some(enemy_owner_index),
            enemy_user_id: Some(enemy_user_id.clone()),
            enemy_vp: Some(enemy_vp),
            enemy_is_bot: is_bot,
            enemy_char_ids: char_ids,
            battle_random_seed: Some(seed),
            created_at: Some(now),
        },
    )
    .await;
    let _ = pvp_user_info::set_current_battle_enemy(pool, uid, Some(enemy_owner_index)).await;

    let enemy_deck_info = deck_info
        .into_iter()
        .map(|d| bd2::proto::proto_net::PvpBattleUserDeckInfo {
            char_inven_index: Some(d.char_inven_index),
            position: Some(d.position),
            sequence: d.sequence,
            costume_inven_index: d.costume_inven_index,
            costume_inven_index_seq: vec![],
            priority_skill_costume_inven_index: vec![],
        })
        .collect();

    let response = PvpBattleMatchingResponse {
        user_vp: Some(user.vp),
        user_rank: Some(rank),
        user_buff_stat_info: vec![],
        enemy_owner_index: Some(enemy_owner_index),
        enemy_user_id: Some(enemy_user_id),
        enemy_exp: Some(0),
        enemy_vp: Some(enemy_vp),
        enemy_rank: None,
        enemy_guild_base_info: None,
        enemy_portrait_costume_id: None,
        enemy_portrait_costume_design_id: None,
        enemy_deck_info,
        enemy_char_info: vec![],
        enemy_costume_info: vec![],
        enemy_equip_info: vec![],
        battle_random_seed: Some(seed),
        event_schedule_info: None,
        enemy_buff_stat_info: vec![],
        enemy_awake_info: vec![],
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PvpBattleMatching.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
