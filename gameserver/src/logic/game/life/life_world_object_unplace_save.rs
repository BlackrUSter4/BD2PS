use super::helper_row_to_dbinfo;
use bd2::prost::Message;
use bd2::proto::proto_net::{LifeWorldObjectUnplaceSaveRequest, LifeWorldObjectUnplaceSaveResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Removing a placed object also clears the work assignment of any helper whose work_id
/// referenced that object's placed index (a workplace being removed out from under them).
pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeWorldObjectUnplaceSaveRequest) -> GameResponse {
    info!("Handling LifeWorldObjectUnplaceSaveRequest: {:?}", req);

    let mut helper_info = Vec::new();

    if let Some(place) = &req.object_unplace_info {
        if let Some(chunk_id) = place.chunk_id {
            for obj in &place.object {
                let Some(object_index) = obj.index else { continue };

                let helpers = database::db::life::life_helper_info::get_by_uid(pool, uid)
                    .await
                    .unwrap_or_default();
                for h in helpers.into_iter().filter(|h| h.work_id == Some(object_index)) {
                    let _ = database::db::life::life_helper_info::clear_work(pool, h.index).await;
                    if let Ok(Some(updated)) =
                        database::db::life::life_helper_info::get_by_slot(pool, uid, h.helper_slot_id.unwrap_or_default())
                            .await
                    {
                        helper_info.push(helper_row_to_dbinfo(&updated));
                    }
                }

                let _ = database::db::life::life_world_object_info::delete_by_object_index(
                    pool,
                    uid,
                    chunk_id,
                    object_index,
                )
                .await;
            }
        }
    }

    let response = LifeWorldObjectUnplaceSaveResponse {
        reward_info_bundle: None,
        helper_info,
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

    let (route, code) = PacketCodeType::LifeWorldObjectUnplaceSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
