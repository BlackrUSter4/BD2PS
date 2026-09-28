use bd2::prost::Message;
use bd2::proto::proto_net::{
    LifeWorldObjectBuildCompletedRequest, LifeWorldObjectBuildCompletedResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Status convention (no Define_ enum exists for this in the schema): 0 = under construction,
/// 1 = completed. Marks every referenced object as completed.
const STATUS_COMPLETED: i32 = 1;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: LifeWorldObjectBuildCompletedRequest,
) -> GameResponse {
    info!("Handling LifeWorldObjectBuildCompletedRequest: {:?}", req);

    let mut result_place = None;

    if let Some(place) = &req.object_place_info {
        if let Some(chunk_id) = place.chunk_id {
            for obj in &place.object {
                if let Some(object_index) = obj.index {
                    if let Ok(Some(existing)) =
                        database::db::life::life_world_object_info::get_by_object_index(
                            pool, uid, chunk_id, object_index,
                        )
                        .await
                    {
                        let _ = database::db::life::life_world_object_info::update_status(
                            pool,
                            existing.index,
                            STATUS_COMPLETED,
                        )
                        .await;
                    }
                }
            }
            let rows = database::db::life::life_world_object_info::get_by_chunk(pool, uid, chunk_id)
                .await
                .unwrap_or_default();
            result_place = super::group_into_place_infos(rows).into_iter().next();
        }
    }

    let response = LifeWorldObjectBuildCompletedResponse {
        object_place_info: result_place,
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

    let (route, code) = PacketCodeType::LifeWorldObjectBuildCompleted.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
