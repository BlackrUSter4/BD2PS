use super::helper_row_to_dbinfo;
use bd2::prost::Message;
use bd2::proto::proto_net::{LifeHelperFireRequest, LifeHelperFireResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// "Fire" = clear the helper's current work assignment (the helper itself stays recruited).
pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeHelperFireRequest) -> GameResponse {
    info!("Handling LifeHelperFireRequest: {:?}", req);

    let mut helper_info = Vec::new();

    if let Some(target) = &req.helper_info {
        let slot = target.helper_slot_id.unwrap_or_default();
        if let Ok(Some(existing)) =
            database::db::life::life_helper_info::get_by_slot(pool, uid, slot).await
        {
            let _ = database::db::life::life_helper_info::clear_work(pool, existing.index).await;
            if let Ok(Some(updated)) =
                database::db::life::life_helper_info::get_by_slot(pool, uid, slot).await
            {
                helper_info.push(helper_row_to_dbinfo(&updated));
            }
        }
    }

    let response = LifeHelperFireResponse {
        reward_bundle: None,
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

    let (route, code) = PacketCodeType::LifeHelperFire.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
