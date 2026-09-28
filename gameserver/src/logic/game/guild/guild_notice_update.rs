use bd2::prost::Message;
use bd2::proto::proto_net::{GuildNoticeUpdateRequest, GuildNoticeUpdateResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::guild_notice_info;
use database::models::game::guild::guild_notice_info::GuildNoticeInfo;
use sqlx::SqlitePool;
use tracing::info;

use super::common::{my_guild, now_ms};

pub async fn handle(pool: &SqlitePool, uid: i64, req: GuildNoticeUpdateRequest) -> GameResponse {
    info!("Handling GuildNoticeUpdateRequest: {:?}", req);
    let now = now_ms();

    if let Some((_info, base)) = my_guild(pool, uid).await {
        let _ = guild_notice_info::delete_guild_notice_info(pool, base.uid).await;
        let row = GuildNoticeInfo {
            index: 0,
            uid: base.uid,
            notice: req.notice.clone(),
            date: Some(now),
        };
        let _ = guild_notice_info::add_guild_notice_info(pool, &row).await;
    }

    let response = GuildNoticeUpdateResponse { date: Some(now) };
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
    let (route, code) = PacketCodeType::GuildNoticeUpdate.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
