use bd2::prost::Message;
use bd2::proto::proto_net::{DatingEpisodeDbInfo, DatingInfoRequest, DatingInfoResponse, DatingMessageChoiceDbInfo, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::dating::{dating_episode_info as episode_db, dating_message_choice_info as choice_db};
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account dating-episode progress and message-choice history (already-scaffolded
/// tables, just needed gameserver glue — was silently returning an empty response before).
pub async fn handle(pool: &SqlitePool, uid: i64, req: DatingInfoRequest) -> GameResponse {
    info!("Handling DatingInfoRequest: {:?}", req);

    let episode_info = episode_db::get_dating_episode_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| DatingEpisodeDbInfo {
            group_id: r.group_id,
            dating_point: r.dating_point,
            last_clear_id: r.last_clear_id,
            last_message_group_id: r.last_message_group_id,
            last_message_id: r.last_message_id,
            last_message_update_time: r.last_message_update_time,
        })
        .collect();

    let message_choice_info = choice_db::get_dating_message_choice_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| DatingMessageChoiceDbInfo { group_id: r.group_id, id: r.id, select_text_id: r.select_text_id })
        .collect();

    let response = DatingInfoResponse { episode_info, message_choice_info };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::DatingInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
