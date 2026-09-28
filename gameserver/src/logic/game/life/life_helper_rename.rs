use bd2::prost::Message;
use bd2::proto::proto_net::{LifeHelperReNameRequest, LifeHelperReNameResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeHelperReNameRequest) -> GameResponse {
    info!("Handling LifeHelperReNameRequest: {:?}", req);

    if let Some(target) = &req.helper_info {
        let slot = target.helper_slot_id.unwrap_or_default();
        if let Some(name) = &target.helper_name {
            if let Ok(Some(existing)) =
                database::db::life::life_helper_info::get_by_slot(pool, uid, slot).await
            {
                let _ = database::db::life::life_helper_info::rename(pool, existing.index, name).await;
            }
        }
    }

    let response = LifeHelperReNameResponse {};
    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::LifeHelperReName.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
