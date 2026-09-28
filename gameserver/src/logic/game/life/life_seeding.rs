use super::now_ms;
use bd2::prost::Message;
use bd2::proto::proto_net::{LifeSeedingRequest, LifeSeedingResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Placeholder growth duration — the real value lives in `LifeCropGradeTable`/
/// `LifeCropSeedTable`, not yet captured.
const PLACEHOLDER_GROWTH_MS: i64 = 60 * 60 * 1000;

pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeSeedingRequest) -> GameResponse {
    info!("Handling LifeSeedingRequest: {:?}", req);

    let now = now_ms();
    let mut touched_places = Vec::new();

    for seeding in &req.seeding_info {
        let Some(place) = &seeding.object_place_info else { continue };
        let Some(chunk_id) = place.chunk_id else { continue };

        if let Some(item) = &seeding.use_item_info {
            if let Some(item_id) = item.id {
                let _ = database::db::item::item_info::consume(pool, uid, item_id, 1).await;
            }
        }

        for obj in &place.object {
            let Some(object_index) = obj.index else { continue };
            if let Ok(Some(existing)) = database::db::life::life_world_object_info::get_by_object_index(
                pool, uid, chunk_id, object_index,
            )
            .await
            {
                let end_time = now + PLACEHOLDER_GROWTH_MS;
                let mut row = existing.clone();
                row.start_time = Some(now);
                row.end_time = Some(end_time);
                let _ = database::db::life::life_world_object_info::upsert(pool, &row).await;
            }
        }
        touched_places.push(chunk_id);
    }

    let mut object_place_info = Vec::new();
    for chunk_id in touched_places {
        let rows = database::db::life::life_world_object_info::get_by_chunk(pool, uid, chunk_id)
            .await
            .unwrap_or_default();
        object_place_info.extend(super::group_into_place_infos(rows));
    }

    let response = LifeSeedingResponse { object_place_info };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::LifeSeeding.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
