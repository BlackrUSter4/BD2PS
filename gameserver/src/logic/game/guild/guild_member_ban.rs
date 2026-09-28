use bd2::prost::Message;
use bd2::proto::proto_net::{GuildMemberBanRequest, GuildMemberBanResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::{guild_info, guild_member_info};
use sqlx::SqlitePool;
use tracing::info;

use super::common::my_guild;

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildMemberBanRequest) -> GameResponse {
    info!("Handling GuildMemberBanRequest: {:?}", req);

    if let (Some(target_owner_index), Some((_info, base))) = (req.owner_index, my_guild(pool, uid).await) {
        let guild_id = base.id.unwrap_or_default();
        let _ = guild_member_info::delete_member_everywhere(pool, guild_id, target_owner_index).await;
        let _ = sqlx::query("DELETE FROM GuildInfo WHERE Uid = ? AND GuildBaseInfoIndex = ?")
            .bind(target_owner_index)
            .bind(guild_id)
            .execute(pool)
            .await;
        let _ = guild_info::update_member_count(pool, guild_id, -1).await;
    }

    let response = GuildMemberBanResponse {};
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
    let (route, code) = PacketCodeType::GuildMemberBan.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
