use super::place_dbinfo_to_row;
use bd2::proto::proto_net::{LifeWorldObjectDecoSaveRequest, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// No response message exists for this request in the schema (client fire-and-forget) — just
/// persist the decoration placement and ack with an empty payload.
pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeWorldObjectDecoSaveRequest) -> GameResponse {
    info!("Handling LifeWorldObjectDecoSaveRequest: {:?}", req);

    if let Some(place) = &req.deco_object_info {
        if let Some(chunk_id) = place.chunk_id {
            for obj in &place.object {
                let row = place_dbinfo_to_row(uid, chunk_id, None, obj);
                let _ = database::db::life::life_world_object_info::upsert(pool, &row).await;
            }
        }
    }

    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::LifeWorldObjectDecoSave.info();
    GameResponse::success(route, &[], code).with_notify(&notify)
}
