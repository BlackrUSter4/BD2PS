use bd2::prost::Message;
use bd2::proto::proto_net::{GuildJoinSendInfoRequest, GuildJoinSendInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::{guild_base_info, guild_info, guild_join_application};
use sqlx::SqlitePool;
use tracing::info;

use super::common::to_guild_db_info;

/// Guilds the caller has a pending (approval-required) application to.
pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildJoinSendInfoRequest) -> GameResponse {
    info!("Handling GuildJoinSendInfoRequest: {:?}", req);

    let applications = guild_join_application::list_for_applicant(pool, uid).await.unwrap_or_default();
    let mut join_send_info = Vec::new();
    for app in applications {
        if let (Ok(Some(base)), Ok(Some(info))) = (
            guild_base_info::get_by_guild_id(pool, app.guild_id).await,
            guild_info::get_by_guild_base_index(pool, app.guild_id).await,
        ) {
            join_send_info.push(to_guild_db_info(&info, &base));
        }
    }

    let response = GuildJoinSendInfoResponse {
        join_send_info,
        action_info: vec![],
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
    let (route, code) = PacketCodeType::GuildJoinRequestInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
