use bd2::prost::Message;
use bd2::proto::proto_net::{GuildAcceptRequest, GuildAcceptResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::guild_join_application;
use sqlx::SqlitePool;
use tracing::info;

use super::common::{my_guild, now_ms};
use super::guild_join::do_join;

/// Guild admin accepts a pending applicant (req.owner_index) into their own guild.
pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildAcceptRequest) -> GameResponse {
    info!("Handling GuildAcceptRequest: {:?}", req);

    if let (Some(applicant_uid), Some((_info, base))) = (req.owner_index, my_guild(pool, uid).await) {
        let guild_id = base.id.unwrap_or_default();
        if guild_join_application::find(pool, guild_id, applicant_uid).await.ok().flatten().is_some() {
            let _ = guild_join_application::delete(pool, guild_id, applicant_uid).await;
            do_join(pool, applicant_uid, guild_id, &base, now_ms()).await;
        }
    }

    let response = GuildAcceptResponse {};
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
    let (route, code) = PacketCodeType::GuildAccept.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
