use bd2::prost::Message;
use bd2::proto::proto_net::{MissionUpdateRequest, MissionUpdateResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mission::mission_info as mission_db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real per-mission progress update, checked against MissionTable's real condition_value.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MissionUpdateRequest) -> GameResponse {
    info!("Handling MissionUpdateRequest: {:?}", req);

    let game_data = data::exceldb::get();
    for entry in &req.update_info {
        let (Some(group_id), Some(id), Some(value)) = (entry.group_id, entry.id, entry.value) else {
            continue;
        };
        let (group_type, is_complete) = match game_data.missiontable.get(id) {
            Some(m) => (m.group_type, value >= m.condition_value),
            None => (None, false),
        };
        let _ = mission_db::upsert_progress(pool, uid, group_id, id, group_type, value, is_complete).await;
    }

    let response = MissionUpdateResponse {};

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MissionUpdate.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
