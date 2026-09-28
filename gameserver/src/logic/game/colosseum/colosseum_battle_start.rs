use bd2::prost::Message;
use bd2::proto::proto_net::{
    ColosseumBattleStartRequest, ColosseumBattleStartResponse, ColosseumDeckInfo as DeckInfoProto,
    ColosseumMatchEnemyInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::colosseum::colosseum_match_candidate;
use rand::Rng;
use sqlx::SqlitePool;
use tracing::info;

use super::{build_deck_info_list, default_notify};

/// Re-resolves the enemy from the candidate matched in the preceding `ColosseumBattleMatching`
/// call (looked up by `enemy_owner_index`, since a bot has no real account to look up).
pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumBattleStartRequest) -> GameResponse {
    info!("Handling ColosseumBattleStartRequest: {:?}", req);

    let enemy_owner_index = req.enemy_owner_index.unwrap_or(0);
    let candidate = colosseum_match_candidate::find(pool, uid, enemy_owner_index).await.ok().flatten();
    let _ = database::db::colosseum::colosseum_user_info::set_current_battle_enemy(
        pool,
        uid,
        Some(enemy_owner_index),
    )
    .await;

    let enemy_info = match candidate {
        Some(c) if !c.enemy_is_bot => {
            let deck = build_deck_info_list(pool, c.enemy_owner_index).await;
            ColosseumMatchEnemyInfo {
                enemy_owner_index: Some(c.enemy_owner_index),
                enemy_user_id: c.enemy_user_id,
                enemy_exp: Some(0),
                enemy_vp: c.enemy_vp,
                enemy_guild_base_info: None,
                enemy_portrait_costume_id: None,
                enemy_portrait_costume_design_id: None,
                enemy_deck_info: deck,
                enemy_char_info: vec![],
                enemy_costume_info: vec![],
                enemy_buff_stat_info: vec![],
                enemy_awake_info: vec![],
                enemy_bless_id: vec![],
                battle_random_seed: Some(rand::thread_rng().gen_range(i32::MIN..=i32::MAX)),
                enemy_top_percent: Some(50.0),
            }
        }
        Some(c) => {
            let bot_chars: Vec<i32> = c
                .enemy_char_ids
                .as_deref()
                .unwrap_or("")
                .split(',')
                .filter_map(|s| s.parse().ok())
                .collect();
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
            ColosseumMatchEnemyInfo {
                enemy_owner_index: Some(c.enemy_owner_index),
                enemy_user_id: c.enemy_user_id,
                enemy_exp: Some(0),
                enemy_vp: c.enemy_vp,
                enemy_guild_base_info: None,
                enemy_portrait_costume_id: bot_chars.first().copied(),
                enemy_portrait_costume_design_id: None,
                enemy_deck_info: bot_deck,
                enemy_char_info: vec![],
                enemy_costume_info: vec![],
                enemy_buff_stat_info: vec![],
                enemy_awake_info: vec![],
                enemy_bless_id: vec![],
                battle_random_seed: Some(rand::thread_rng().gen_range(i32::MIN..=i32::MAX)),
                enemy_top_percent: Some(50.0),
            }
        }
        // Candidate expired/not found (e.g. client retried without re-matching) — degrade
        // gracefully rather than error, with just the requested owner index carried through.
        None => ColosseumMatchEnemyInfo {
            enemy_owner_index: Some(enemy_owner_index),
            battle_random_seed: Some(rand::thread_rng().gen_range(i32::MIN..=i32::MAX)),
            ..Default::default()
        },
    };

    let response = ColosseumBattleStartResponse {
        user_buff_stat_info: vec![],
        event_schedule_info: None,
        enemy_info: Some(enemy_info),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumBattleStart.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
