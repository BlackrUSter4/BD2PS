use bd2::prost::Message;
use bd2::proto::proto_net::{GuildRaidSeasonRankDbInfo, GuildRaidSeasonRankingRequest, GuildRaidSeasonRankingResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::{guild_base_info, guild_info, guild_raid_main_info};
use sqlx::SqlitePool;
use tracing::info;

use super::common::my_guild;

/// Ranks every real guild on this server by its creator account's own
/// GuildRaidMainInfo.guild_total_score (the only per-guild aggregate this
/// schema tracks).
pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildRaidSeasonRankingRequest) -> GameResponse {
    info!("Handling GuildRaidSeasonRankingRequest: {:?}", req);

    let bases = guild_base_info::get_all_guilds(pool, 100).await.unwrap_or_default();
    let mut scored = Vec::new();
    for base in bases {
        let guild_id = base.id.unwrap_or_default();
        let score = guild_raid_main_info::get_guild_raid_main_info(pool, base.uid)
            .await
            .ok()
            .and_then(|v| v.into_iter().next())
            .and_then(|m| m.guild_total_score)
            .unwrap_or(0);
        let member_count = guild_info::get_by_guild_base_index(pool, guild_id)
            .await
            .ok()
            .flatten()
            .and_then(|i| i.member_count);
        scored.push((base, score, member_count));
    }
    scored.sort_by(|a, b| b.1.cmp(&a.1));

    let mut rank_info = Vec::new();
    for (rank, (base, score, member_count)) in scored.into_iter().enumerate() {
        rank_info.push(GuildRaidSeasonRankDbInfo {
            rank: Some(rank as i32 + 1),
            score: Some(score),
            guild_index: base.id,
            guild_name: base.name,
            message: None,
            icon: base.icon,
            icon_color: base.icon_color,
            flag_grade: None,
            member_count,
            over_kill_damage: None,
        });
    }

    let user_rank_info = if let Some((_info, base)) = my_guild(pool, uid).await {
        rank_info.iter().find(|r| r.guild_index == base.id).cloned()
    } else {
        None
    };

    let response = GuildRaidSeasonRankingResponse { rank_info, user_rank_info };
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
    // No dedicated PacketCodeType exists for this route.
    let (route, code) = PacketCodeType::Common.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
