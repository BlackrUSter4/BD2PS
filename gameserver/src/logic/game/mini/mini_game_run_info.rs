use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameDbInfo, MiniGameRunInfoRequest, MiniGameRunInfoResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_run_info as db;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::info;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct RunRecord {
    pub event_schedule_id: i32,
    pub best_record_value: i32,
}

pub fn parse_records(info_index: &Option<String>) -> Vec<RunRecord> {
    info_index.as_deref().and_then(|s| serde_json::from_str(s).ok()).unwrap_or_default()
}

/// Real per-account best-score read, same JSON-blob-in-InfoIndex design as Field's equivalent.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameRunInfoRequest) -> GameResponse {
    info!("Handling MiniGameRunInfoRequest: {:?}", req);

    let row = db::get_mini_game_run_info(pool, uid).await.unwrap_or_default().into_iter().next();
    let records = row.map(|r| parse_records(&r.info_index)).unwrap_or_default();

    let info = records
        .into_iter()
        .filter(|r| req.event_schedule_id.is_empty() || req.event_schedule_id.contains(&r.event_schedule_id))
        .map(|r| MiniGameDbInfo {
            event_schedule_id: Some(r.event_schedule_id),
            best_record_value: Some(r.best_record_value),
            last_reward_point: Some(0),
            is_possible_quick_reward: Some(false),
            ..Default::default()
        })
        .collect();

    let response = MiniGameRunInfoResponse { info };

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

    let (route, code) = PacketCodeType::MiniGameRunInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
