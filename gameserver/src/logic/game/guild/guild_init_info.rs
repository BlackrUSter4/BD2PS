use bd2::prost::Message;
use bd2::proto::proto_net::{GuildActionDbInfo, GuildInitInfoRequest, GuildInitInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::guild_action_info;
use database::db::guild::guild_raid_play_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildInitInfoRequest) -> GameResponse {
    info!("Handling GuildInitInfoRequest: {:?}", req);

    let action_info: Vec<GuildActionDbInfo> = guild_action_info::get_guild_action_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|a| GuildActionDbInfo {
            r#type: a.r#type.as_ref().and_then(|v| v.as_i64()).map(|v| v as i32),
            time: a.time,
            guild_id: a.guild_id,
            guild_name: a.guild_name,
            is_notify: a.is_notify.map(|v| v != 0),
            role: a.role.as_ref().and_then(|v| v.as_i64()).map(|v| v as i32),
        })
        .collect();

    let raid_play_info = guild_raid_play_info::get_guild_raid_play_info(pool, uid)
        .await
        .ok()
        .and_then(|v| v.into_iter().next())
        .map(|r| bd2::proto::proto_net::GuildRaidPlayDbInfo {
            boss_score: r.boss_score,
            total_score: r.total_score,
            top_percent: r.top_percent,
            is_play_raid_today: r.is_play_raid_today.map(|v| v != 0),
            is_normal_battle_play: r.is_normal_battle_play.map(|v| v != 0),
            battle_mode: r.battle_mode.as_ref().and_then(|v| v.as_i64()).map(|v| v as i32),
            rank: r.rank,
        });

    let response = GuildInitInfoResponse {
        join_recv_info: vec![],
        action_info,
        is_reward: Some(false),
        raid_play_info,
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

    let (route, code) = PacketCodeType::GuildInitInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
