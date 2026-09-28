use bd2::prost::Message;
use bd2::proto::proto_net::{
    CharDbInfo, ColosseumBattleMatchingRequest, ColosseumBattleMatchingResponse,
    ColosseumDeckInfo as DeckInfoProto, ColosseumMatchEnemyInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{char::char_info, colosseum::{colosseum_match_candidate, colosseum_user_info}};
use rand::seq::IndexedRandom;
use rand::Rng;
use sqlx::SqlitePool;
use tracing::info;

use super::{build_deck_info_list, default_notify, now_ms, MATCH_CANDIDATE_COUNT};

/// Real accounts are matched against their actual saved deck when at least one other real
/// account exists in this server's own database, with a real `enemy_char_info` snapshot built
/// from that account's own CharInfo rows. Otherwise (the common case for a single-account
/// private-server instance) a bot opponent is synthesized from real `CharTable` character ids
/// near the caller's own Vp — there's no real opponent to show otherwise, and an empty match
/// list would make Colosseum entirely unplayable. Bots never get a real char/costume/buff
/// snapshot (there's no account behind them to look one up from) — `enemy_char_info`/
/// `enemy_costume_info`/etc. stay empty for them. `enemy_costume_info`/`enemy_buff_stat_info`/
/// `enemy_awake_info` stay empty even for real opponents too — those need real stat-calculation
/// formulas this project has never modeled anywhere, not just a database lookup, so fabricating
/// them would be inventing game balance rather than reading real data.
pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumBattleMatchingRequest) -> GameResponse {
    info!("Handling ColosseumBattleMatchingRequest: {:?}", req);

    let user = colosseum_user_info::get_or_create(pool, uid).await.unwrap_or_default();
    let rank = colosseum_user_info::rank_of(pool, uid).await.unwrap_or(1);

    let real_opponents = colosseum_user_info::find_nearby_opponents(pool, uid, MATCH_CANDIDATE_COUNT)
        .await
        .unwrap_or_default();

    let mut candidates = Vec::new();
    let mut enemy_infos = Vec::new();
    let now = now_ms();

    for opp in &real_opponents {
        let opp_deck = build_deck_info_list(pool, opp.uid).await;
        let account = database::db::user::user::find_account(pool, opp.uid).await.ok().flatten();
        let user_id = account.map(|a| a.user_name).unwrap_or_else(|| format!("Player{}", opp.uid));

        let mut opp_char_info = Vec::with_capacity(opp_deck.len());
        for d in &opp_deck {
            if let Some(idx) = d.char_inven_index {
                if let Ok(Some(row)) = char_info::get_by_inven_index(pool, opp.uid, idx).await {
                    opp_char_info.push(CharDbInfo {
                        inven_index: row.inven_index,
                        id: row.id,
                        hp: row.hp,
                        level: row.level,
                        costume_id: row.costume_id,
                        exp: row.exp,
                        use_costume: row.use_costume,
                        talent_level: row.talent_level,
                        talent_exp: row.talent_exp,
                        solidarity_reward: row.solidarity_reward,
                        expiry_time: row.expiry_time,
                        pictorialbook_info: vec![],
                        connect_potential_costume: row.connect_potential_costume,
                    });
                }
            }
        }

        candidates.push(colosseum_match_candidate::Candidate {
            enemy_owner_index: opp.uid,
            enemy_user_id: Some(user_id.clone()),
            enemy_vp: Some(opp.vp),
            enemy_is_bot: false,
            enemy_char_ids: None,
        });
        enemy_infos.push(ColosseumMatchEnemyInfo {
            enemy_owner_index: Some(opp.uid),
            enemy_user_id: Some(user_id),
            enemy_exp: Some(0),
            enemy_vp: Some(opp.vp),
            enemy_guild_base_info: None,
            enemy_portrait_costume_id: None,
            enemy_portrait_costume_design_id: None,
            enemy_deck_info: opp_deck,
            enemy_char_info: opp_char_info,
            enemy_costume_info: vec![],
            enemy_buff_stat_info: vec![],
            enemy_awake_info: vec![],
            enemy_bless_id: vec![],
            battle_random_seed: Some(rand::thread_rng().gen_range(i32::MIN..=i32::MAX)),
            enemy_top_percent: Some(50.0),
        });
    }

    let missing = (MATCH_CANDIDATE_COUNT as usize).saturating_sub(real_opponents.len());
    if missing > 0 {
        let all_chars = &data::exceldb::get().chartable;
        let mut rng = rand::thread_rng();
        let pool_ids: Vec<i32> = all_chars.all().iter().map(|c| c.id).collect();

        for i in 0..missing {
            let bot_owner_index = -(now / 1000 + i as i64 + 1); // negative sentinel: never a real uid
            let vp_jitter = rng.gen_range(-100..=100);
            let bot_vp = (user.vp + vp_jitter).max(0);
            let bot_chars: Vec<i32> = if pool_ids.is_empty() {
                vec![]
            } else {
                pool_ids.choose_multiple(&mut rng, 5.min(pool_ids.len())).copied().collect()
            };
            let bot_name = format!("Guest{}", rng.gen_range(1000..9999));
            let char_ids_csv = bot_chars.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(",");

            let bot_deck: Vec<DeckInfoProto> = bot_chars
                .iter()
                .enumerate()
                .map(|(pos, _)| DeckInfoProto {
                    char_inven_index: Some(-(pos as i64 + 1)),
                    position: Some(pos as i32),
                    sequence: Some(0),
                    costume_inven_index: None,
                })
                .collect();

            candidates.push(colosseum_match_candidate::Candidate {
                enemy_owner_index: bot_owner_index,
                enemy_user_id: Some(bot_name.clone()),
                enemy_vp: Some(bot_vp),
                enemy_is_bot: true,
                enemy_char_ids: Some(char_ids_csv),
            });
            enemy_infos.push(ColosseumMatchEnemyInfo {
                enemy_owner_index: Some(bot_owner_index),
                enemy_user_id: Some(bot_name),
                enemy_exp: Some(0),
                enemy_vp: Some(bot_vp),
                enemy_guild_base_info: None,
                enemy_portrait_costume_id: bot_chars.first().copied(),
                enemy_portrait_costume_design_id: None,
                enemy_deck_info: bot_deck,
                enemy_char_info: vec![],
                enemy_costume_info: vec![],
                enemy_buff_stat_info: vec![],
                enemy_awake_info: vec![],
                enemy_bless_id: vec![],
                battle_random_seed: Some(rng.gen_range(i32::MIN..=i32::MAX)),
                enemy_top_percent: Some(50.0),
            });
        }
    }

    let _ = colosseum_match_candidate::replace_all(pool, uid, &candidates, now).await;
    let reload_count = if req.is_reload.unwrap_or(false) {
        colosseum_user_info::incr_match_reroll(pool, uid).await.unwrap_or(0)
    } else {
        user.match_reroll_count
    };

    let response = ColosseumBattleMatchingResponse {
        user_vp: Some(user.vp),
        user_rank: Some(rank),
        enemy_info: enemy_infos,
        first_attack_team: Some(0),
        match_reload_count: Some(reload_count),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumBattleMatching.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
