use bd2::prost::Message;
use bd2::proto::proto_net::{
    EventRewardHistoryDbInfo, EventRewardHistoryRequest, EventRewardHistoryResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::event::event_reward_history_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account claimed-reward history, grouped by (event_schedule_id, event_group_id).
/// Legitimately empty until the account has actually claimed an event reward.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EventRewardHistoryRequest) -> GameResponse {
    info!("Handling EventRewardHistoryRequest: {:?}", req);

    let rows = db::get_event_reward_history_info(pool, uid)
        .await
        .unwrap_or_default();

    let mut by_group: std::collections::BTreeMap<(i32, i32), Vec<i32>> =
        std::collections::BTreeMap::new();
    for r in rows {
        by_group
            .entry((r.event_schedule_id.unwrap_or(0), r.event_group_id.unwrap_or(0)))
            .or_default()
            .push(r.reward_id);
    }

    let history_info = by_group
        .into_iter()
        .map(|((event_schedule_id, event_group_id), reward_id)| EventRewardHistoryDbInfo {
            reward_id,
            event_schedule_id: Some(event_schedule_id),
            event_group_id: Some(event_group_id),
        })
        .collect();

    let response = EventRewardHistoryResponse { history_info };

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

    let (route, code) = PacketCodeType::EventRewardHistory.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
