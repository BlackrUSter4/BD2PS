use bd2::prost::Message;
use bd2::proto::proto_net::{LifeWorldObjectStatusSaveRequest, LifeWorldObjectStatusSaveResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeWorldObjectStatusSaveRequest) -> GameResponse {
    info!("Handling LifeWorldObjectStatusSaveRequest: {:?}", req);

    let mut result_places = Vec::new();

    for place in &req.object_place_info {
        let Some(chunk_id) = place.chunk_id else { continue };
        for obj in &place.object {
            let Some(object_index) = obj.index else { continue };
            let Some(status) = obj.status else { continue };
            if let Ok(Some(existing)) = database::db::life::life_world_object_info::get_by_object_index(
                pool, uid, chunk_id, object_index,
            )
            .await
            {
                let _ =
                    database::db::life::life_world_object_info::update_status(pool, existing.index, status)
                        .await;
            }
        }
        let rows = database::db::life::life_world_object_info::get_by_chunk(pool, uid, chunk_id)
            .await
            .unwrap_or_default();
        result_places.extend(super::group_into_place_infos(rows));
    }

    let response = LifeWorldObjectStatusSaveResponse {
        object_place_info: result_places,
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

    let (route, code) = PacketCodeType::LifeWorldObjectStatusSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
