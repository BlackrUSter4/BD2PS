use super::now_ms;
use bd2::prost::Message;
use bd2::proto::proto_net::{
    LifeWorldObjectGatheringRequest, LifeWorldObjectGatheringResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Marks ready (end_time already elapsed) objects as gathered (clears their timer) and unlocks
/// a collection entry per distinct object_id gathered. The actual yield (`LifeGatheringObjectTable`)
/// isn't captured, so the reward bundle is left empty.
pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeWorldObjectGatheringRequest) -> GameResponse {
    info!("Handling LifeWorldObjectGatheringRequest: {:?}", req);

    let now = now_ms();
    let mut new_collection_id = Vec::new();
    let mut result_place = None;

    for place in &req.object_place_info {
        let Some(chunk_id) = place.chunk_id else { continue };
        for obj in &place.object {
            let Some(object_index) = obj.index else { continue };
            let Ok(Some(existing)) = database::db::life::life_world_object_info::get_by_object_index(
                pool, uid, chunk_id, object_index,
            )
            .await
            else {
                continue;
            };
            let ready = existing.end_time.map(|t| now >= t).unwrap_or(true);
            if !ready {
                continue;
            }
            let mut row = existing.clone();
            row.start_time = None;
            row.end_time = None;
            let _ = database::db::life::life_world_object_info::upsert(pool, &row).await;

            if let Some(object_id) = existing.object_id {
                if database::db::life::life_collection_info::add_if_missing(pool, uid, object_id)
                    .await
                    .unwrap_or(false)
                {
                    new_collection_id.push(object_id);
                }
            }
        }

        let rows = database::db::life::life_world_object_info::get_by_chunk(pool, uid, chunk_id)
            .await
            .unwrap_or_default();
        result_place = super::group_into_place_infos(rows).into_iter().next();
    }

    let response = LifeWorldObjectGatheringResponse {
        life_char_level_info: None,
        reward_info_bundle: None,
        object_place_info: result_place,
        new_collection_id,
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::LifeWorldObjectGathering.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
