use bd2::prost::Message;
use bd2::proto::proto_net::{GuildRaidMainInfoRequest, GuildRaidMainInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::guild_raid_main_info;
use database::models::game::guild::guild_raid_main_info::GuildRaidMainInfo;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildRaidMainInfoRequest) -> GameResponse {
    info!("Handling GuildRaidMainInfoRequest: {:?}", req);

    let row = match guild_raid_main_info::get_guild_raid_main_info(pool, uid).await.ok().and_then(|v| v.into_iter().next()) {
        Some(r) => r,
        None => {
            let fresh = GuildRaidMainInfo {
                index: 0,
                uid,
                season: Some(1),
                raid_day: Some(1),
                today_normal_battle_count: Some(0),
                user_score: Some(0),
                last_score_reward_id: None,
                guild_total_score: Some(0),
                guild_top_percent: Some(0.0),
                golem_level: Some(1),
                golem_exp: Some(0),
                obtainable_season_reward: Some(0),
                today_supporter_use_count: Some(0),
                total_supporter_rental_count: Some(0),
                top_guild_score: Some(0),
                play_day: 1,
                schedule_history_info_index: None,
                flag_grade_version_info_index: None,
                guild_rank: None,
            };
            let _ = guild_raid_main_info::add_guild_raid_main_info(pool, &fresh).await;
            fresh
        }
    };

    let response = GuildRaidMainInfoResponse {
        season: row.season,
        raid_day: row.raid_day,
        today_normal_battle_count: row.today_normal_battle_count,
        user_score: row.user_score,
        last_score_reward_id: row.last_score_reward_id,
        guild_total_score: row.guild_total_score,
        guild_top_percent: row.guild_top_percent,
        golem_level: row.golem_level,
        golem_exp: row.golem_exp,
        obtainable_season_reward: row.obtainable_season_reward.map(|v| v != 0),
        today_supporter_use_count: row.today_supporter_use_count,
        total_supporter_rental_count: row.total_supporter_rental_count,
        top_guild_score: row.top_guild_score,
        play_day: vec![row.play_day],
        guild_rank: row.guild_rank,
        ..Default::default()
    };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::GuildRaidMainInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
