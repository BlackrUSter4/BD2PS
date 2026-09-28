use bd2::prost::Message;
use bd2::proto::proto_net::{GuildRaidNormalBattleDbInfo, GuildRaidNormalBattleInfoRequest, GuildRaidNormalBattleInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::{guild_raid_main_info, guild_raid_normal_battle_info};
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildRaidNormalBattleInfoRequest) -> GameResponse {
    info!("Handling GuildRaidNormalBattleInfoRequest: {:?}", req);

    let info_list: Vec<GuildRaidNormalBattleDbInfo> = guild_raid_normal_battle_info::get_guild_raid_normal_battle_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| GuildRaidNormalBattleDbInfo {
            group_id: r.group_id,
            id: r.id,
            level: r.level,
            complete_win_count: r.complete_win_count,
            battle_challenge_index: vec![r.battle_challenge_index],
        })
        .collect();

    let main = guild_raid_main_info::get_guild_raid_main_info(pool, uid)
        .await
        .ok()
        .and_then(|v| v.into_iter().next());

    let response = GuildRaidNormalBattleInfoResponse {
        info: info_list,
        golem_level: main.as_ref().and_then(|m| m.golem_level),
        golem_exp: main.as_ref().and_then(|m| m.golem_exp),
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
    let (route, code) = PacketCodeType::GuildRaidNormalBattleInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
