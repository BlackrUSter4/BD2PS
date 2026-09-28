use super::to_dbinfo;
use bd2::prost::Message;
use bd2::proto::proto_net::{EquipStorageInfoRequest, EquipStorageInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipStorageInfoRequest) -> GameResponse {
    info!("Handling EquipStorageInfoRequest: {:?}", req);

    let stored = database::db::equip::equip_storage_info::get_stored_indices(pool, uid)
        .await
        .unwrap_or_default();
    let equips = database::db::equip::equip_info::get_all_by_inven_index(pool, uid, &stored)
        .await
        .unwrap_or_default();
    let base_indices: Vec<i64> = equips.iter().filter_map(|e| e.base_info_index).collect();
    let bases = database::db::equip::equip_base_info::get_all_by_index(pool, uid, &base_indices)
        .await
        .unwrap_or_default();
    let equip_info = equips
        .iter()
        .map(|e| {
            let base = bases.iter().find(|b| Some(b.index) == e.base_info_index);
            to_dbinfo(e, base)
        })
        .collect();

    let response = EquipStorageInfoResponse { equip_info };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipStorageInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
