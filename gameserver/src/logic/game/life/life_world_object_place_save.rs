use super::{place_dbinfo_to_row, try_consume_items};
use bd2::prost::Message;
use bd2::proto::proto_net::{LifeWorldObjectPlaceSaveRequest, LifeWorldObjectPlaceSaveResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeWorldObjectPlaceSaveRequest) -> GameResponse {
    info!("Handling LifeWorldObjectPlaceSaveRequest: {:?}", req);

    let _paid = try_consume_items(pool, uid, &req.use_item_info).await;

    let mut result_place = None;
    if let Some(place) = &req.object_place_info {
        if let Some(chunk_id) = place.chunk_id {
            for obj in &place.object {
                let row = place_dbinfo_to_row(uid, chunk_id, None, obj);
                let _ = database::db::life::life_world_object_info::upsert(pool, &row).await;
            }
            let rows = database::db::life::life_world_object_info::get_by_chunk(pool, uid, chunk_id)
                .await
                .unwrap_or_default();
            result_place = super::group_into_place_infos(rows).into_iter().next();
        }
    }

    let response = LifeWorldObjectPlaceSaveResponse {
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

    let (route, code) = PacketCodeType::LifeWorldObjectPlaceSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
