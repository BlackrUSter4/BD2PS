use super::{get_equip_with_base, to_dbinfo};
use bd2::prost::Message;
use bd2::proto::proto_net::{EquipStoragePutRequest, EquipStoragePutResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Moves an equip into storage (adds its inven_index to the stored set). Storage is tracked as
/// a tag layered on top of the normal equip inventory rather than removing the row entirely —
/// EquipInfo/EquipStorageInfo aren't set up as separate item stores in this schema.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipStoragePutRequest) -> GameResponse {
    info!("Handling EquipStoragePutRequest: {:?}", req);

    let mut equip_info = None;
    if let Some(inven_index) = req.equip_info.as_ref().and_then(|e| e.inven_index) {
        let mut stored = database::db::equip::equip_storage_info::get_stored_indices(pool, uid)
            .await
            .unwrap_or_default();
        if !stored.contains(&inven_index) {
            stored.push(inven_index);
            let _ = database::db::equip::equip_storage_info::set_stored_indices(pool, uid, &stored)
                .await;
        }
        if let Some((equip, base)) = get_equip_with_base(pool, uid, inven_index).await {
            equip_info = Some(to_dbinfo(&equip, base.as_ref()));
        }
    }

    let response = EquipStoragePutResponse { equip_info };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipStoragePut.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
