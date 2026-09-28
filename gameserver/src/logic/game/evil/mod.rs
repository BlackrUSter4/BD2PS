pub mod evil_castle_daily_reward;
pub mod evil_castle_daily_reward_state;
pub mod evil_castle_enter;
pub mod evil_castle_give_up;
pub mod evil_castle_info;
pub mod evil_castle_ping;
pub mod evil_castle_ranking_info;
pub mod evil_castle_reward_info;
pub mod evil_castle_rogue_like_all_user_score_info;
pub mod evil_castle_rogue_like_battle_char_change;
pub mod evil_castle_rogue_like_battle_end_preview;
pub mod evil_castle_rogue_like_battle_skip;
pub mod evil_castle_rogue_like_char_revival;
pub mod evil_castle_rogue_like_costume_upgrade;
pub mod evil_castle_rogue_like_deck_save;
pub mod evil_castle_rogue_like_edit_event;
pub mod evil_castle_rogue_like_enter;
pub mod evil_castle_rogue_like_event_choice;
pub mod evil_castle_rogue_like_event_select;
pub mod evil_castle_rogue_like_give_up;
pub mod evil_castle_rogue_like_give_up_info;
pub mod evil_castle_rogue_like_growth;
pub mod evil_castle_rogue_like_info;
pub mod evil_castle_rogue_like_move_floor;
pub mod evil_castle_rogue_like_quick_battle;
pub mod evil_castle_rogue_like_rank_info;
pub mod evil_castle_rogue_like_relic_mix;
pub mod evil_castle_rogue_like_reward_re_roll;
pub mod evil_castle_rogue_like_room_enter;
pub mod evil_castle_rogue_like_season_reward;
pub mod evil_castle_rogue_like_select_reward;
pub mod evil_castle_rogue_like_shop_buy;
pub mod evil_castle_rogue_like_shop_exit;
pub mod evil_castle_rogue_like_shop_re_roll;
pub mod evil_castle_stage_clear_reward;
pub mod evil_castle_stage_ranking_info;
pub mod evil_castle_tower_info;
pub mod roguelike;

use bd2::proto::proto_net::{ItemDbInfo, RewardDbInfoBundle};
use database::db::item::item_info;
use sqlx::SqlitePool;

/// Shared reward-granting helper: applies parallel id/type/count arrays (the
/// shape every EvilCastle reward table uses — EvilCastleTable, RLRewardTable,
/// EvilCastleDailyRewardTable, RLLevelTable) via the same generic
/// `item_info::grant` path the Battle round established, and returns the
/// resulting bundle for the response.
pub async fn grant_rewards(
    pool: &SqlitePool,
    uid: i64,
    ids: &[i32],
    types: &[i32],
    counts: &[i32],
) -> RewardDbInfoBundle {
    let mut item_infos = vec![];
    let n = ids.len().min(types.len()).min(counts.len());
    for i in 0..n {
        let (id, r#type, count) = (ids[i], types[i], counts[i]);
        if item_info::grant(pool, uid, id, r#type, count).await.is_ok() {
            if let Ok(Some(item)) = item_info::find_by_item_id(pool, uid, id).await {
                item_infos.push(ItemDbInfo {
                    inven_index: item.inven_index,
                    id: item.id,
                    r#type: item.r#type,
                    count: item.count,
                    keep_flag: item.keep_flag,
                    time_value: item.time_value,
                    pictorialbook_info: None,
                    expiry_time: item.expiry_time,
                    sort_id: item.sort_id,
                    use_count: item.use_count,
                });
            }
        }
    }
    RewardDbInfoBundle {
        item_info: item_infos,
        ..Default::default()
    }
}
