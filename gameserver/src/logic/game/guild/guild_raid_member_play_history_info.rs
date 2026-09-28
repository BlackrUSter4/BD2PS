use bd2::prost::Message;
use bd2::proto::proto_net::{GuildRaidMemberPlayHistoryDbInfo, GuildRaidMemberPlayHistoryInfoRequest, GuildRaidMemberPlayHistoryInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::guild_raid_member_play_history_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildRaidMemberPlayHistoryInfoRequest) -> GameResponse {
    info!("Handling GuildRaidMemberPlayHistoryInfoRequest: {:?}", req);

    let target = req.target_owner_index.unwrap_or(uid);
    let info_list: Vec<GuildRaidMemberPlayHistoryDbInfo> = guild_raid_member_play_history_info::get_guild_raid_member_play_history_info(pool, target)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter(|r| req.season.is_none() || r.raid_day.is_some())
        .map(|r| GuildRaidMemberPlayHistoryDbInfo { raid_day: r.raid_day, level: r.level })
        .collect();

    let response = GuildRaidMemberPlayHistoryInfoResponse { info: info_list };
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
    let (route, code) = PacketCodeType::GuildRaidMemberPlayHistoryInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
