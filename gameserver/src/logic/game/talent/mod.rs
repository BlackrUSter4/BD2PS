pub mod talent_skill_upgrade;
pub mod talent_skill_use;
pub mod talent_slot_save;

use data::exceldb;
use database::db::char::char_info;
use sqlx::SqlitePool;

/// Real TalentLevel/TalentExp progression, cross-checked line-for-line against the reference
/// server's `GameTalentServer.EquipMaking`/`Alchemy`/`TalentSkillUse` (all three share this exact
/// mechanic): NeedExp to reach the next talent level is the SUM of `TalentGrowthTable.needExp`
/// for every level 1..=current (within the character's own `TalentTable.growth_group_id`;
/// `TalentGrowthTable.id` is the level itself within its group, 1-indexed, confirmed from the
/// captured data — e.g. group 105 has ids 1..5 with increasing needExp, id 5 at max level having
/// no needExp at all, meaning "fully grown"). Per-use/per-craft exp gained is
/// `TalentSkillTable.getExp` at the same (`talent_skill_group_id`, current level), multiplied by
/// `use_count`. Exp clamps at the threshold without auto-leveling — the reference leaves the
/// actual level bump to a separate TalentSkillUpgrade call, matched here. Returns the real exp
/// actually added (0 if the character/talent data can't be resolved).
pub async fn add_talent_exp(pool: &SqlitePool, uid: i64, inven_index: i64, use_count: i32) -> i32 {
    let Ok(Some(char_row)) = char_info::get_by_inven_index(pool, uid, inven_index).await else {
        return 0;
    };
    let Some(char_id) = char_row.id else {
        return 0;
    };
    let db = exceldb::get();
    let Some(talent_id) = db.chartable.get(char_id).and_then(|c| c.talent_id) else {
        return 0;
    };
    let Some(talent) = db.talenttable.get(talent_id) else {
        return 0;
    };

    let current_level = char_row.talent_level.unwrap_or(0).max(1);
    let need_exp: i32 = (1..=current_level)
        .filter_map(|lvl| {
            db.talentgrowthtable
                .by_group(talent.growth_group_id)
                .find(|r| r.id == lvl)
                .and_then(|r| r.need_exp)
        })
        .sum();
    let get_exp = db
        .talentskilltable
        .by_group(talent.talent_skill_group_id)
        .find(|r| r.id == current_level)
        .and_then(|r| r.get_exp)
        .unwrap_or(0);

    let current_exp = char_row.talent_exp.unwrap_or(0);
    let proposed = current_exp + get_exp * use_count.max(1);
    let (new_exp, added) = if proposed >= need_exp {
        (need_exp, need_exp - current_exp)
    } else {
        (proposed, get_exp * use_count.max(1))
    };

    let _ = char_info::set_talent_level_exp(pool, uid, inven_index, current_level, new_exp).await;
    added
}
