use bd2::prost::Message;
use bd2::proto::proto_net::{FieldEventSpawnCaughtInfo, FieldEventSpawnInfoRequest, FieldEventSpawnInfoResponse, FieldEventSpawnProgressInfo as ProgressProto};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::field_event_spawn::{field_event_spawn_daily_count, field_event_spawn_progress_info as progress_db};
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: FieldEventSpawnInfoRequest) -> GameResponse {
    info!("Handling FieldEventSpawnInfoRequest: {:?}", req);

    let current_progress = progress_db::get(pool, uid).await.map(|row| ProgressProto {
        start_time: row.start_time,
        event_schedule_id: row.event_schedule_id,
        spawn_event_id: row.spawn_event_id,
        group_id: row.group_id,
        caught_info: super::parse_caught(&row.caught_info)
            .into_iter()
            .map(|c| FieldEventSpawnCaughtInfo {
                spawn_event_id: Some(c.spawn_event_id),
                monster_group_id: Some(c.monster_group_id),
                monster_id: Some(c.monster_id),
            })
            .collect(),
    });

    let today = chrono::Utc::now().date_naive().to_string();
    let daily = field_event_spawn_daily_count::get(pool, uid, &today).await;

    let response = FieldEventSpawnInfoResponse {
        current_progress,
        daily_normal_count: Some(daily.normal_count),
        daily_special_count: Some(daily.special_count),
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::FieldEventSpawnInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
