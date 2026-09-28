use bd2::prost::Message;
use bd2::proto::proto_net::{
    MonsterHuntUserDbInfo, MonsterHuntUserInfoRequest, MonsterHuntUserInfoResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::monster::monster_hunt_user_info as db;
use database::models::game::monster::monster_hunt_user_info::MonsterHuntUserInfo;
use sqlx::SqlitePool;
use tracing::info;

use super::{boss_hp_at_level, default_boss_id, default_notify};

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: MonsterHuntUserInfoRequest,
) -> GameResponse {
    info!("Handling MonsterHuntUserInfoRequest: {:?}", req);

    let boss_id = default_boss_id();
    let row = match db::get_or_create(pool, uid, || {
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
            tracing::error!("MonsterHuntUserInfo get_or_create failed: {}", e);
            return GameResponse::error(1);
        }
    };

    let (rank, rank_top_percent) = super::compute_rank(pool, uid).await;

    let response = MonsterHuntUserInfoResponse {
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
        rank: Some(rank),
        rank_top_percent: Some(rank_top_percent),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MonsterHuntUserInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
