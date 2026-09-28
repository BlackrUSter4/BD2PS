use bd2::prost::Message;
use bd2::proto::proto_net::{
    ItemDbInfo, MonsterHuntQuickBattleRequest, MonsterHuntQuickBattleResponse,
    MonsterHuntUserDbInfo, RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, monster::monster_hunt_user_info as db};
use database::models::game::monster::monster_hunt_user_info::MonsterHuntUserInfo;
use sqlx::SqlitePool;
use tracing::info;

use super::{boss_hp_at_level, default_boss_id, default_notify, GOLD_ITEM_ID, GOLD_ITEM_TYPE};

/// One quick-battle "attempt" deals this much damage. No table anywhere states a per-attempt
/// damage formula (deck/character stats -> damage isn't modeled server-side at all in this
/// project), so this is a documented placeholder flat roll. Everything else here — whether
/// the boss actually dies at this level (compared against the REAL data-driven HP curve in
/// `boss_hp_at_level`), leveling up, and reward granting — is real.
const PLACEHOLDER_DAMAGE_ROLL: i64 = 5000;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: MonsterHuntQuickBattleRequest,
) -> GameResponse {
    info!("Handling MonsterHuntQuickBattleRequest: {:?}", req);

    let boss_id = default_boss_id();
    let mut row = match db::get_or_create(pool, uid, || {
        let hp = boss_hp_at_level(boss_id, 1);
        MonsterHuntUserInfo {
            index: 0,
            uid,
            season: Some(1),
            monster_hunt_id: Some(boss_id),
            level: Some(1),
            start_hp: Some(hp),
            highest_first_turn_damage: Some(0),
            highest_hp: Some(hp),
            highest_hp_date: Some(chrono::Utc::now().timestamp_millis()),
            current_level_highest_damage: Some(0),
            daily_highest_damage: Some(0),
            season_reward: Some(false),
            daily_reward_level: Some(0),
            daily_reward_date: Some(0),
        }
    })
    .await
    {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("MonsterHuntQuickBattle get_or_create failed: {}", e);
            return GameResponse::error(1);
        }
    };

    let level = row.level.unwrap_or(1);
    let remaining = boss_hp_at_level(boss_id, level)
        - row.current_level_highest_damage.unwrap_or(0).max(0);
    let damage = PLACEHOLDER_DAMAGE_ROLL;

    let mut item_infos = Vec::new();
    let cap = data::exceldb::get()
        .monsterhunttable
        .get(boss_id)
        .map(|b| b.reward_level)
        .unwrap_or(level);

    if damage >= remaining && level < cap {
        // Boss defeated at this level: level up and grant the real per-level reward.
        let new_level = level + 1;
        row.level = Some(new_level);
        row.current_level_highest_damage = Some(0);
        row.highest_hp = Some(boss_hp_at_level(boss_id, new_level));
        row.highest_hp_date = Some(chrono::Utc::now().timestamp_millis());

        if let Some(reward_group) = data::exceldb::get().monsterhunttable.get(boss_id).map(|b| b.reward_group_id) {
            if let Some(reward) = data::exceldb::get()
                .monsterhuntrewardtable
                .by_group(reward_group)
                .find(|r| r.level == new_level)
                .cloned()
            {
                for i in 0..reward.reward_id.len() {
                    let id = reward.reward_id[i];
                    let ty = *reward.reward_type.get(i).unwrap_or(&GOLD_ITEM_TYPE);
                    let count = *reward.reward_count.get(i).unwrap_or(&0);
                    if count > 0 {
                        let _ = item_info::grant(pool, uid, id, ty, count).await;
                        item_infos.push(ItemDbInfo {
                            id: Some(id),
                            r#type: Some(ty),
                            count: Some(count),
                            ..Default::default()
                        });
                    }
                }
            }
        }
    } else {
        row.current_level_highest_damage =
            Some(row.current_level_highest_damage.unwrap_or(0).max(damage));
    }

    row.daily_highest_damage = Some(row.daily_highest_damage.unwrap_or(0).max(damage));
    row.highest_first_turn_damage =
        Some(row.highest_first_turn_damage.unwrap_or(0).max(damage as i32));

    // Real daily reward: once per calendar day, using this level's real dailyReward* triple.
    let today = chrono::Utc::now().date_naive();
    let last_claim = row
        .daily_reward_date
        .filter(|&t| t > 0)
        .and_then(|t| chrono::DateTime::from_timestamp_millis(t))
        .map(|d| d.date_naive());
    let mut daily_items = Vec::new();
    if last_claim != Some(today) {
        if let Some(reward_group) = data::exceldb::get().monsterhunttable.get(boss_id).map(|b| b.reward_group_id) {
            if let Some(reward) = data::exceldb::get()
                .monsterhuntrewardtable
                .by_group(reward_group)
                .find(|r| r.level == row.level.unwrap_or(level))
                .cloned()
            {
                for i in 0..reward.daily_reward_id.len() {
                    let id = reward.daily_reward_id[i];
                    let ty = *reward.daily_reward_type.get(i).unwrap_or(&GOLD_ITEM_TYPE);
                    let count = *reward.daily_reward_count.get(i).unwrap_or(&0);
                    let grant_id = if id == 0 { GOLD_ITEM_ID } else { id };
                    if count > 0 {
                        let _ = item_info::grant(pool, uid, grant_id, ty, count).await;
                        daily_items.push(ItemDbInfo {
                            id: Some(grant_id),
                            r#type: Some(ty),
                            count: Some(count),
                            ..Default::default()
                        });
                    }
                }
                row.daily_reward_date = Some(chrono::Utc::now().timestamp_millis());
                row.daily_reward_level = row.level;
            }
        }
    }

    if let Err(e) = db::update_monster_hunt_user_info(pool, &row).await {
        tracing::error!("MonsterHuntQuickBattle update failed: {}", e);
    }

    let (rank, rank_top_percent) = super::compute_rank(pool, uid).await;

    let response = MonsterHuntQuickBattleResponse {
        monster_hunt_user_info: Some(MonsterHuntUserDbInfo {
            season: row.season,
            monster_hunt_id: row.monster_hunt_id,
            level: row.level,
            start_hp: row.start_hp,
            highest_first_turn_damage: row.highest_first_turn_damage,
            highest_hp: row.highest_hp,
            highest_hp_date: row.highest_hp_date,
            current_level_highest_damage: row.current_level_highest_damage,
            daily_highest_damage: row.daily_highest_damage,
            season_reward: row.season_reward,
            daily_reward_level: row.daily_reward_level,
            daily_reward_date: row.daily_reward_date,
        }),
        monster_hunt_reward_bundle: Some(RewardDbInfoBundle {
            item_info: item_infos,
            ..Default::default()
        }),
        monster_hunt_daily_reward_bundle: Some(RewardDbInfoBundle {
            item_info: daily_items,
            ..Default::default()
        }),
        rank: Some(rank),
        rank_top_percent: Some(rank_top_percent),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MonsterHuntQuickBattle.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
