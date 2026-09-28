use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameRunEndRequest, MiniGameRunEndResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_run_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::mini_game_run_info::{parse_records, RunRecord};

/// Real best-record persistence (JSON blob keyed by event_schedule_id — this request has no
/// such field though, so a single default-slot record is used; documented limitation). No
/// run-reward master table identified this pass, so last_reward_point stays 0.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameRunEndRequest) -> GameResponse {
    info!("Handling MiniGameRunEndRequest: {:?}", req);

    let row = db::get_mini_game_run_info(pool, uid).await.unwrap_or_default().into_iter().next();
    let mut records = row.map(|r| parse_records(&r.info_index)).unwrap_or_default();

    let new_value = req.record_value.unwrap_or(0);
    let entry = records.iter_mut().find(|r| r.event_schedule_id == 0);
    let best = match entry {
        Some(r) => {
            if new_value > r.best_record_value {
                r.best_record_value = new_value;
            }
            r.best_record_value
        }
        None => {
            records.push(RunRecord { event_schedule_id: 0, best_record_value: new_value });
            new_value
        }
    };

    let _ = db::set_info_index(pool, uid, &serde_json::to_string(&records).unwrap_or_default()).await;

    let response = MiniGameRunEndResponse {
        best_record_value: Some(best),
        last_reward_point: Some(0),
        reward_info_bundle: None,
        is_possible_quick_reward: Some(false),
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
    
    let (route, code) = PacketCodeType::MiniGameRunEnd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}