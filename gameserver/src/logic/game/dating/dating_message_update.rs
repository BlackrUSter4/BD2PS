use bd2::prost::Message;
use bd2::proto::proto_net::{DatingEpisodeDbInfo, DatingMessageChoiceDbInfo, DatingMessageUpdateRequest, DatingMessageUpdateResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::dating::{dating_episode_info as episode_db, dating_message_choice_info as choice_db};
use sqlx::SqlitePool;
use tracing::info;

/// Real message-choice persistence and real last-seen-message tracking.
pub async fn handle(pool: &SqlitePool, uid: i64, req: DatingMessageUpdateRequest) -> GameResponse {
    info!("Handling DatingMessageUpdateRequest: {:?}", req);

    let mut episode_info = None;
    let mut message_choice_info = None;

    if let (Some(group_id), Some(id)) = (req.group_id, req.id) {
        let now = chrono::Utc::now().timestamp_millis();
        let _ = episode_db::get_or_create(pool, uid, group_id).await;
        let _ = episode_db::set_last_message(pool, uid, group_id, group_id, id, now).await;

        if let Some(select_text_id) = req.select_text_id {
            let _ = choice_db::upsert(pool, uid, group_id, id, select_text_id).await;
            message_choice_info = Some(DatingMessageChoiceDbInfo { group_id: Some(group_id), id: Some(id), select_text_id: Some(select_text_id) });
        }

        episode_info = episode_db::get_by_group(pool, uid, group_id)
            .await
            .ok()
            .flatten()
            .map(|r| DatingEpisodeDbInfo {
                group_id: r.group_id,
                dating_point: r.dating_point,
                last_clear_id: r.last_clear_id,
                last_message_group_id: r.last_message_group_id,
                last_message_id: r.last_message_id,
                last_message_update_time: r.last_message_update_time,
            });
    }

    let response = DatingMessageUpdateResponse { episode_info, message_choice_info };

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

    let (route, code) = PacketCodeType::DatingMessageUpdate.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
