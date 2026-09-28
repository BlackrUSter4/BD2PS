use bd2::proto::proto_net::{GuildBaseDbInfo, GuildDbInfo, GuildMemberDbInfo};
use database::db::guild::guild_base_info as base_db;
use database::db::guild::guild_info as info_db;
use database::db::guild::guild_member_info as member_db;
use database::models::game::guild::guild_base_info::GuildBaseInfo;
use database::models::game::guild::guild_info::GuildInfo;
use sqlx::SqlitePool;

/// Find the guild the given account currently belongs to (its own synced
/// GuildInfo row), then load that guild's canonical GuildBaseInfo row
/// (which may live under a different account's Uid if they didn't found it).
pub async fn my_guild(pool: &SqlitePool, uid: i64) -> Option<(GuildInfo, GuildBaseInfo)> {
    let info_rows = info_db::get_guild_info(pool, uid).await.ok()?;
    let info = info_rows.into_iter().next()?;
    let guild_id = info.guild_base_info_index?;
    let base = base_db::get_by_guild_id(pool, guild_id).await.ok()??;
    Some((info, base))
}

pub fn to_guild_base_db_info(base: &GuildBaseInfo) -> GuildBaseDbInfo {
    GuildBaseDbInfo {
        id: base.id,
        name: base.name.clone(),
        icon: base.icon,
        icon_color: base.icon_color.clone(),
        grade: base.grade,
    }
}

pub fn to_guild_db_info(info: &GuildInfo, base: &GuildBaseInfo) -> GuildDbInfo {
    GuildDbInfo {
        guild_base_info: Some(to_guild_base_db_info(base)),
        join_type: info
            .join_type
            .as_ref()
            .and_then(|v| v.as_i64())
            .map(|v| v as i32),
        message: info.message.clone(),
        update_date: info.update_date,
        date: info.date,
        member_count: info.member_count,
        delete_remaining_time: info.delete_remaining_time,
        notice_update_date: info.notice_update_date,
    }
}

/// Full roster for a guild, read from the canonical creator-synced copy
/// (every join/leave/ban writes through the creator's own Uid as well as the
/// acting account's, so the guild id's rows are consistent across accounts).
/// `user_id` is looked up fresh here (real in-game nickname, see
/// `logic::game::display_name`) rather than trusted from the stored row — the stored value
/// is just whatever it was at join/creation time and would go stale if the member later
/// changes their nickname via the User profile-settings route.
pub async fn member_list(pool: &SqlitePool, guild_id: i64) -> Vec<GuildMemberDbInfo> {
    let rows = member_db::get_by_guild_id_any_uid(pool, guild_id)
        .await
        .unwrap_or_default();
    let mut out = Vec::with_capacity(rows.len());
    for m in rows {
        let user_id = match m.owner_index {
            Some(owner) => Some(crate::logic::game::display_name(pool, owner).await),
            None => m.user_id,
        };
        out.push(GuildMemberDbInfo {
            id: m.id,
            owner_index: m.owner_index,
            user_id,
            title_id: m.title_id,
            portrait_costume_id: m.portrait_costume_id,
            portrait_costume_design_id: m.portrait_costume_design_id,
            role: m.role.as_ref().and_then(|v| v.as_i64()).map(|v| v as i32),
            point: m.point,
            supporter_info: vec![],
            last_login_date: m.last_login_date,
            update_date: m.update_date,
            date: m.date,
        });
    }
    out
}

pub fn now_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

use bd2::proto::proto_net::PresetDbInfo;
use database::db::guild::guild_raid_preset_info;
use database::models::game::guild::guild_raid_preset_info::GuildRaidPresetInfo;

/// The scaffolded schema stores the whole preset list as one opaque TEXT
/// column rather than a normalized child table — repurposed to hold a
/// serialized JSON array, keyed within it by each preset's own `slot` field.
pub async fn load_presets(pool: &SqlitePool, uid: i64) -> Vec<PresetDbInfo> {
    guild_raid_preset_info::get_guild_raid_preset_info(pool, uid)
        .await
        .ok()
        .and_then(|v| v.into_iter().next())
        .and_then(|r| r.preset_info_index)
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub async fn save_presets(pool: &SqlitePool, uid: i64, presets: &[PresetDbInfo]) {
    let _ = guild_raid_preset_info::delete_guild_raid_preset_info(pool, uid).await;
    let row = GuildRaidPresetInfo {
        index: 0,
        uid,
        preset_info_index: Some(serde_json::to_string(presets).unwrap_or_default()),
    };
    let _ = guild_raid_preset_info::add_guild_raid_preset_info(pool, &row).await;
}
