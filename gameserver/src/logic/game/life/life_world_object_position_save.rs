use bd2::proto::proto_net::{LifeWorldObjectPositionSaveRequest, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Each move_info carries a before/after full place-info pair — we only need the "after" state
/// to persist the new position (the "before" is just what the client had cached).
pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeWorldObjectPositionSaveRequest) -> GameResponse {
    info!("Handling LifeWorldObjectPositionSaveRequest: {:?}", req);

    for m in &req.move_info {
        let Some(after) = &m.after_object_place_info else { continue };
        let Some(chunk_id) = after.chunk_id else { continue };
        for obj in &after.object {
            let Some(object_index) = obj.index else { continue };
            if let Ok(Some(existing)) = database::db::life::life_world_object_info::get_by_object_index(
                pool, uid, chunk_id, object_index,
            )
            .await
            {
                let _ = database::db::life::life_world_object_info::update_position(
                    pool,
                    existing.index,
                    obj.x.unwrap_or(existing.x.unwrap_or(0)),
                    obj.y.unwrap_or(existing.y.unwrap_or(0)),
                    obj.rotate.or(existing.rotate),
                )
                .await;
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

    let (route, code) = PacketCodeType::LifeWorldObjectPositionSave.info();
    GameResponse::success(route, &[], code).with_notify(&notify)
}
