use bd2::prost::Message;
use bd2::proto::proto_net::{
    ItemDbInfo, MonsterHuntSeasonRewardRequest, MonsterHuntSeasonRewardResponse, RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, monster::monster_hunt_user_info as db};
use sqlx::SqlitePool;
use tracing::info;

use super::{default_boss_id, default_notify, GOLD_ITEM_TYPE};

/// Claim-once season-end reward based on real cross-account rank, using MonsterHuntRankTable's
/// real ranking-threshold tiers (top 1 / top 10 / top 100) and their real reward triples.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: MonsterHuntSeasonRewardRequest,
) -> GameResponse {
    info!("Handling MonsterHuntSeasonRewardRequest: {:?}", req);

    let boss_id = default_boss_id();
    let all = db::rank_all(pool, 100000).await.unwrap_or_default();
    let total = all.len().max(1);
    let rank = all.iter().position(|r| r.uid == uid).map(|p| p + 1).unwrap_or(total);
    let rank_top_percent = (rank as f64 / total as f64) * 100.0;

    let mut item_infos = Vec::new();

    if let Ok(mut row) = db::get_or_create(pool, uid, || {
        database::models::game::monster::monster_hunt_user_info::MonsterHuntUserInfo {
            index: 0,
            uid,
            season: Some(1),
            monster_hunt_id: Some(boss_id),
            level: Some(1),
            start_hp: Some(super::boss_hp_at_level(boss_id, 1)),
            highest_first_turn_damage: Some(0),
            highest_hp: Some(super::boss_hp_at_level(boss_id, 1)),
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
        if !row.season_reward.unwrap_or(false) {
            let tier = data::exceldb::get()
                .monsterhuntranktable
                .by_group(1)
                .filter(|t| (rank as f32) <= t.ranking)
                .min_by(|a, b| a.ranking.partial_cmp(&b.ranking).unwrap());

            if let Some(tier) = tier.cloned() {
                for i in 0..tier.season_reward_id.len() {
                    let id = tier.season_reward_id[i];
                    let ty = *tier.season_reward_type.get(i).unwrap_or(&GOLD_ITEM_TYPE);
                    let count = *tier.season_reward_count.get(i).unwrap_or(&0);
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
            row.season_reward = Some(true);
            let _ = db::update_monster_hunt_user_info(pool, &row).await;
        }
    }

    let response = MonsterHuntSeasonRewardResponse {
        rank: Some(rank as i32),
        score: Some(rank as i32),
        monster_hunt_id: Some(boss_id),
        reward_info_bundle: Some(RewardDbInfoBundle {
            item_info: item_infos,
            ..Default::default()
        }),
        rank_top_percent: Some(rank_top_percent),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MonsterHuntSeasonReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
