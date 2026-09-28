use bd2::prost::Message;
use bd2::proto::proto_net::{EquipBatchUseRequest, EquipBatchUseResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipBatchUseRequest) -> GameResponse {
    info!("Handling EquipBatchUseRequest: {:?}", req);

    for batch in &req.equip_batch_info {
        if let Some(char_index) = batch.char_inven_index {
            for equip_index in &batch.equip_inven_index {
                let _ = database::db::equip::equip_info::set_use_char(
                    pool,
                    uid,
                    *equip_index,
                    Some(char_index),
                )
                .await;
            }
        }
    }

    let response = EquipBatchUseResponse { char_info: vec![] };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipBatchUse.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
