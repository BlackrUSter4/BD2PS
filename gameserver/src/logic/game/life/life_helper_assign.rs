use super::{helper_row_to_dbinfo, now_ms};
use bd2::prost::Message;
use bd2::proto::proto_net::{LifeHelperAssignRequest, LifeHelperAssignResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeHelperAssignRequest) -> GameResponse {
    info!("Handling LifeHelperAssignRequest: {:?}", req);

    let mut helper_info = Vec::new();

    if let Some(target) = &req.helper_info {
        let slot = target.helper_slot_id.unwrap_or_default();
        if let Ok(Some(existing)) =
            database::db::life::life_helper_info::get_by_slot(pool, uid, slot).await
        {
            let work_type = target.work_type.unwrap_or_default();
            let work_id = target.work_id.unwrap_or_default();
            let now = now_ms();
            let _ = database::db::life::life_helper_info::assign_work(
                pool,
                existing.index,
                work_type,
                work_id,
                now,
            )
            .await;

            if let Ok(Some(updated)) =
                database::db::life::life_helper_info::get_by_slot(pool, uid, slot).await
            {
                helper_info.push(helper_row_to_dbinfo(&updated));
            }
        }
    }

    let response = LifeHelperAssignResponse {
        helper_info,
        reward_bundle: None,
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

    let (route, code) = PacketCodeType::LifeHelperAssign.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
