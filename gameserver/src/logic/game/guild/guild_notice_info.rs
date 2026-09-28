use bd2::prost::Message;
use bd2::proto::proto_net::{GuildNoticeInfoRequest, GuildNoticeInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::guild_notice_info;
use sqlx::SqlitePool;
use tracing::info;

use super::common::my_guild;

/// The notice lives under the guild's owner (creator) Uid — the only shared
/// per-guild identity this schema has, since GuildNoticeInfo has no guild id
/// column of its own.
pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildNoticeInfoRequest) -> GameResponse {
    info!("Handling GuildNoticeInfoRequest: {:?}", req);

    let (notice, date) = if let Some((_info, base)) = my_guild(pool, uid).await {
        guild_notice_info::get_guild_notice_info(pool, base.uid)
            .await
            .ok()
            .and_then(|v| v.into_iter().next())
            .map(|n| (n.notice, n.date))
            .unwrap_or((None, None))
    } else {
        (None, None)
    };

    let response = GuildNoticeInfoResponse { notice, date };
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
    let (route, code) = PacketCodeType::GuildNoticeInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
