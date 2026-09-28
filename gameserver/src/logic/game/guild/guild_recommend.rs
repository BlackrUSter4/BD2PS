use bd2::prost::Message;
use bd2::proto::proto_net::{GuildRecommendRequest, GuildRecommendResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::guild::{guild_base_info, guild_info};
use sqlx::SqlitePool;
use tracing::info;

use super::common::to_guild_db_info;

pub async fn handle(pool: &SqlitePool, _uid: i64, req: GuildRecommendRequest) -> GameResponse {
    info!("Handling GuildRecommendRequest: {:?}", req);

    let bases = guild_base_info::get_all_guilds(pool, 20).await.unwrap_or_default();
    let mut guild_info_list = Vec::new();
    for base in bases {
        let guild_id = base.id.unwrap_or_default();
        if let Some(info) = guild_info::get_by_guild_base_index(pool, guild_id).await.ok().flatten() {
            let count = info.member_count.unwrap_or(0);
            if let (Some(min), true) = (req.search_min, count < req.search_min.unwrap_or(0)) {
                let _ = min;
                continue;
            }
            if let (Some(max), true) = (req.search_max, count > req.search_max.unwrap_or(i32::MAX)) {
                let _ = max;
                continue;
            }
            guild_info_list.push(to_guild_db_info(&info, &base));
        }
    }

    let response = GuildRecommendResponse { guild_info: guild_info_list };
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
    let (route, code) = PacketCodeType::GuildRecommend.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
