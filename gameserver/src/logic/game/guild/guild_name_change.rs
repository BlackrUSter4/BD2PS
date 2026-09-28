use bd2::prost::Message;
use bd2::proto::proto_net::{GuildNameChangeRequest, GuildNameChangeResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::{guild_base_info, guild_member_info};
use sqlx::SqlitePool;
use tracing::info;

/// Real rename of the caller's own guild (found via their GuildMemberInfo row's guild id).
pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildNameChangeRequest) -> GameResponse {
    info!("Handling GuildNameChangeRequest: {:?}", req);

    if let Ok(members) = guild_member_info::get_guild_member_info(pool, uid).await {
        if let Some(guild_id) = members.first().and_then(|m| m.id) {
            let _ = guild_base_info::update_by_guild_id(pool, guild_id, req.name.as_deref(), None, None).await;
        }
    }

    let response = GuildNameChangeResponse {};
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
    let (route, code) = PacketCodeType::GuildNameChange.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
