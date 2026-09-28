use bd2::prost::Message;
use bd2::proto::proto_net::{GuildMemberRoleEditRequest, GuildMemberRoleEditResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::guild_member_info;
use sqlx::SqlitePool;
use tracing::info;

use super::common::{member_list, my_guild};

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildMemberRoleEditRequest) -> GameResponse {
    info!("Handling GuildMemberRoleEditRequest: {:?}", req);

    let mut member_info = Vec::new();
    if let (Some(target_owner_index), Some(role), Some((_info, base))) =
        (req.owner_index, req.role, my_guild(pool, uid).await)
    {
        let guild_id = base.id.unwrap_or_default();
        let _ = guild_member_info::update_role(pool, guild_id, target_owner_index, role).await;
        member_info = member_list(pool, guild_id).await;
    }

    let response = GuildMemberRoleEditResponse { member_info };
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
    let (route, code) = PacketCodeType::GuildMemberRoleEdit.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
