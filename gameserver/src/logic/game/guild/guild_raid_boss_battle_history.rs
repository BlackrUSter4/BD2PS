use bd2::prost::Message;
use bd2::proto::proto_net::{GuildRaidBossBattleHistoryRequest, GuildRaidBossBattleHistoryResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::guild_raid_boss_battle_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildRaidBossBattleHistoryRequest) -> GameResponse {
    info!("Handling GuildRaidBossBattleHistoryRequest: {:?}", req);

    let row = guild_raid_boss_battle_info::get_guild_raid_boss_battle_info(pool, uid)
        .await
        .ok()
        .and_then(|v| v.into_iter().next());

    let response = GuildRaidBossBattleHistoryResponse {
        today_highest_level: row.as_ref().and_then(|r| r.highest_level),
        today_highest_score: row.as_ref().and_then(|r| r.highest_score),
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
    let (route, code) = PacketCodeType::GuildRaidBossBattleHistory.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
