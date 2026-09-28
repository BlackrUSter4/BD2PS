use bd2::prost::Message;
use bd2::proto::proto_net::{FieldEventSpawnProgressInfo as ProgressProto, FieldEventSpawnStartRequest, FieldEventSpawnStartResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::field_event_spawn::field_event_spawn_progress_info as db;
use database::models::game::field_event_spawn::field_event_spawn_progress_info::FieldEventSpawnProgressInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Real progress tracking: starts (or restarts) the caller's spawn-hunt session for the
/// requested event/group, clearing any prior caught list.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FieldEventSpawnStartRequest) -> GameResponse {
    info!("Handling FieldEventSpawnStartRequest: {:?}", req);

    let now = chrono::Utc::now().timestamp_millis();
    let row = FieldEventSpawnProgressInfo {
        uid,
        start_time: Some(now),
        event_schedule_id: req.event_schedule_id,
        spawn_event_id: req.id,
        group_id: req.group_id,
        caught_info: None,
    };
    let _ = db::save(pool, &row).await;

    let progress = Some(ProgressProto {
        start_time: row.start_time,
        event_schedule_id: row.event_schedule_id,
        spawn_event_id: row.spawn_event_id,
        group_id: row.group_id,
        caught_info: vec![],
    });

    let response = FieldEventSpawnStartResponse { progress };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::FieldEventSpawnStart.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
