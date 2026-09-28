use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameDbInfo, MiniGameFieldInfoRequest, MiniGameFieldInfoResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_field_info as db;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::info;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct FieldRecord {
    pub event_schedule_id: i32,
    pub last_reward_point: i32,
    pub best_record_value: i32,
}

pub fn parse_records(info_index: &Option<String>) -> Vec<FieldRecord> {
    info_index.as_deref().and_then(|s| serde_json::from_str(s).ok()).unwrap_or_default()
}

/// Real per-account best-score read. The pre-existing `InfoIndex` TEXT column (a vague
/// index-list design, same anti-pattern fixed elsewhere this session) is reused as a small
/// JSON blob of per-event records instead of adding a new migration for a 6-handler cluster.
/// Cross-account world-best fields aren't attempted this pass — left honestly empty.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameFieldInfoRequest) -> GameResponse {
    info!("Handling MiniGameFieldInfoRequest: {:?}", req);

    let row = db::get_mini_game_field_info(pool, uid).await.unwrap_or_default().into_iter().next();
    let records = row.map(|r| parse_records(&r.info_index)).unwrap_or_default();

    let info = records
        .into_iter()
        .filter(|r| req.event_schedule_id.is_empty() || req.event_schedule_id.contains(&r.event_schedule_id))
        .map(|r| MiniGameDbInfo {
            event_schedule_id: Some(r.event_schedule_id),
            last_reward_point: Some(r.last_reward_point),
            best_record_value: Some(r.best_record_value),
            is_possible_quick_reward: Some(false),
            world_best_record_owner_index: None,
            world_best_record_user_id: None,
            ..Default::default()
        })
        .collect();

    let response = MiniGameFieldInfoResponse { info };

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

    let (route, code) = PacketCodeType::MiniGameFieldInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
