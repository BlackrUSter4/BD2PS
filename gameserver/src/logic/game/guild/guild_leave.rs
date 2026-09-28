use bd2::prost::Message;
use bd2::proto::proto_net::{GuildLeaveRequest, GuildLeaveResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::{guild_info, guild_member_info};
use sqlx::SqlitePool;
use tracing::info;

use super::common::my_guild;

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildLeaveRequest) -> GameResponse {
    info!("Handling GuildLeaveRequest: {:?}", req);

    if let Some((_info, base)) = my_guild(pool, uid).await {
        let guild_id = base.id.unwrap_or_default();
        let _ = guild_member_info::delete_member_everywhere(pool, guild_id, uid).await;
        let _ = guild_info::update_member_count(pool, guild_id, -1).await;
        let _ = guild_info::delete_guild_info(pool, uid).await;
    }

    let response = GuildLeaveResponse {};
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

    let (route, code) = PacketCodeType::GuildLeave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
