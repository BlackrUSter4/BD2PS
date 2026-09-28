use bd2::proto::proto_net::{LifeWorldObjectPositionDeltaSaveRequest, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: LifeWorldObjectPositionDeltaSaveRequest,
) -> GameResponse {
    info!("Handling LifeWorldObjectPositionDeltaSaveRequest: {:?}", req);

    let dx = req.dx.unwrap_or(0);
    let dy = req.dy.unwrap_or(0);

    for m in &req.move_info {
        let Some(chunk_id) = m.chunk_id else { continue };
        let Some(object_id) = m.object_id else { continue };
        // object_id here identifies the row by its placed ObjectId (not a per-slot index) —
        // this request moves everything matching by delta rather than naming a specific index.
        let rows = database::db::life::life_world_object_info::get_by_chunk(pool, uid, chunk_id)
            .await
            .unwrap_or_default();
        for row in rows.into_iter().filter(|r| r.object_id == Some(object_id)) {
            let new_x = row.x.unwrap_or(0) + dx;
            let new_y = row.y.unwrap_or(0) + dy;
            let rotate = m.rotate.or(row.rotate);
            let _ =
                database::db::life::life_world_object_info::update_position(pool, row.index, new_x, new_y, rotate)
                    .await;
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

    let (route, code) = PacketCodeType::LifeWorldObjectPositionDeltaSave.info();
    GameResponse::success(route, &[], code).with_notify(&notify)
}
