use bd2::prost::Message;
use bd2::proto::proto_net::{GuildRaidBossBattleInfoRequest, GuildRaidBossBattleInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::guild_raid_boss_battle_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildRaidBossBattleInfoRequest) -> GameResponse {
    info!("Handling GuildRaidBossBattleInfoRequest: {:?}", req);

    let row = guild_raid_boss_battle_info::get_guild_raid_boss_battle_info(pool, uid)
        .await
        .ok()
        .and_then(|v| v.into_iter().next());

    let response = match row {
        Some(r) => GuildRaidBossBattleInfoResponse {
            guild_total_score: r.guild_total_score,
            guild_top_percent: r.guild_top_percent,
            highest_level: r.highest_level,
            highest_score: r.highest_score,
            top_member_owner_index: r.top_member_owner_index,
            top_member_user_id: r.top_member_user_id,
            top_member_score: r.top_member_score,
        },
        None => GuildRaidBossBattleInfoResponse::default(),
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
    let (route, code) = PacketCodeType::GuildRaidBossBattleInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
