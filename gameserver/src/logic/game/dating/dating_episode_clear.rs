use bd2::prost::Message;
use bd2::proto::proto_net::{DatingEpisodeClearRequest, DatingEpisodeClearResponse, DatingEpisodeDbInfo, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::dating::dating_episode_info as episode_db;
use sqlx::SqlitePool;
use tracing::info;

/// Real claim-once episode clear (only advances if this id is actually newer than the
/// account's real last_clear_id). No dating-episode reward table was captured anywhere in
/// this project — reward_info_bundle stays honestly empty rather than fabricated.
pub async fn handle(pool: &SqlitePool, uid: i64, req: DatingEpisodeClearRequest) -> GameResponse {
    info!("Handling DatingEpisodeClearRequest: {:?}", req);

    let mut episode_info = None;
    if let (Some(group_id), Some(id)) = (req.group_id, req.id) {
        let row = episode_db::get_or_create(pool, uid, group_id).await.ok();
        if let Some(row) = row {
            if id > row.last_clear_id.unwrap_or(0) {
                let _ = episode_db::set_last_clear_id(pool, uid, group_id, id).await;
            }
            let updated = episode_db::get_by_group(pool, uid, group_id).await.ok().flatten();
            episode_info = updated.map(|r| DatingEpisodeDbInfo {
                group_id: r.group_id,
                dating_point: r.dating_point,
                last_clear_id: r.last_clear_id,
                last_message_group_id: r.last_message_group_id,
                last_message_id: r.last_message_id,
                last_message_update_time: r.last_message_update_time,
            });
        }
    }

    let response = DatingEpisodeClearResponse {
        reward_info_bundle: Some(RewardDbInfoBundle::default()),
        episode_info,
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

    let (route, code) = PacketCodeType::DatingEpisodeClear.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
