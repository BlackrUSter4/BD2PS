use bd2::prost::Message;
use bd2::proto::proto_net::{GuildInfoRequest, GuildInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::{guild_base_info, guild_info, guild_raid_play_info};
use sqlx::SqlitePool;
use tracing::info;

use super::common::{member_list, my_guild, to_guild_db_info};

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildInfoRequest) -> GameResponse {
    info!("Handling GuildInfoRequest: {:?}", req);

    let resolved = if let Some(id) = req.id {
        match (
            guild_base_info::get_by_guild_id(pool, id).await.ok().flatten(),
            guild_info::get_by_guild_base_index(pool, id).await.ok().flatten(),
        ) {
            (Some(base), Some(info)) => Some((info, base)),
            _ => None,
        }
    } else {
        my_guild(pool, uid).await
    };

    let (guild_db_info, member_info, guild_id) = match resolved {
        Some((info, base)) => {
            let id = base.id.unwrap_or_default();
            (Some(to_guild_db_info(&info, &base)), member_list(pool, id).await, Some(id))
        }
        None => (None, vec![], None),
    };

    let raid_play_info = if let Some(_gid) = guild_id {
        guild_raid_play_info::get_guild_raid_play_info(pool, uid)
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
            })
    } else {
        None
    };

    let response = GuildInfoResponse {
        guild_info: guild_db_info,
        member_info,
        join_recv_info: vec![],
        action_info: vec![],
        is_attendance: Some(false),
        raid_play_info,
        ban_time: None,
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

    let (route, code) = PacketCodeType::GuildInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
