use super::{get_equip_with_base, to_dbinfo};
use bd2::prost::Message;
use bd2::proto::proto_net::{EquipStorageOutRequest, EquipStorageOutResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipStorageOutRequest) -> GameResponse {
    info!("Handling EquipStorageOutRequest: {:?}", req);

    let mut equip_info = None;
    if let Some(inven_index) = req.equip_info.as_ref().and_then(|e| e.inven_index) {
        let stored = database::db::equip::equip_storage_info::get_stored_indices(pool, uid)
            .await
            .unwrap_or_default();
        let new_stored: Vec<i64> = stored.into_iter().filter(|i| *i != inven_index).collect();
        let _ =
            database::db::equip::equip_storage_info::set_stored_indices(pool, uid, &new_stored)
                .await;
        if let Some((equip, base)) = get_equip_with_base(pool, uid, inven_index).await {
            equip_info = Some(to_dbinfo(&equip, base.as_ref()));
        }
    }

    let response = EquipStorageOutResponse { equip_info };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipStorageOut.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
