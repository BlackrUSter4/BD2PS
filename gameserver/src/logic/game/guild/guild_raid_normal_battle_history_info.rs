use bd2::prost::Message;
use bd2::proto::proto_net::{GuildRaidNormalBattleHistoryDbInfo, GuildRaidNormalBattleHistoryInfoRequest, GuildRaidNormalBattleHistoryInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::guild_raid_normal_battle_history_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildRaidNormalBattleHistoryInfoRequest) -> GameResponse {
    info!("Handling GuildRaidNormalBattleHistoryInfoRequest: {:?}", req);

    let info_list: Vec<GuildRaidNormalBattleHistoryDbInfo> = guild_raid_normal_battle_history_info::get_guild_raid_normal_battle_history_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| GuildRaidNormalBattleHistoryDbInfo {
            owner_index: r.owner_index,
            user_id: r.user_id,
            portrait_costume_id: r.portrait_costume_id,
            portrait_costume_design_id: r.portrait_costume_design_id,
            title_id: r.title_id,
        })
        .collect();

    let response = GuildRaidNormalBattleHistoryInfoResponse { info: info_list };
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
    let (route, code) = PacketCodeType::GuildRaidNormalBattleHistoryInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
