use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameFieldEndRequest, MiniGameFieldEndResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_field_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::mini_game_field_info::{parse_records, FieldRecord};

/// Real per-account best-score persistence (see mini_game_field_info.rs for the JSON-blob
/// design note). No reward table identified this pass, so last_reward_point stays 0.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameFieldEndRequest) -> GameResponse {
    info!("Handling MiniGameFieldEndRequest: {:?}", req);

    // This request has no event_schedule_id/score field (only is_force_end), matching the
    // schema exactly — there's nothing real to record here, so the current best is echoed
    // back unchanged rather than fabricating a score.
    let row = db::get_mini_game_field_info(pool, uid).await.unwrap_or_default().into_iter().next();
    let records = row.map(|r| parse_records(&r.info_index)).unwrap_or_default();
    let best = records.into_iter().max_by_key(|r| r.best_record_value).unwrap_or(FieldRecord::default());

    let response = MiniGameFieldEndResponse {
        event_schedule_id: Some(best.event_schedule_id),
        score: Some(best.best_record_value),
        highest_score: Some(best.best_record_value),
        last_reward_point: Some(0),
        is_possible_quick_reward: Some(false),
        reward_info_bundle: None,
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
    
    let (route, code) = PacketCodeType::MiniGameFieldEnd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}