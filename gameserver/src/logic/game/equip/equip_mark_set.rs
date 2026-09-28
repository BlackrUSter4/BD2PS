use bd2::prost::Message;
use bd2::proto::proto_net::{EquipMarkSetRequest, EquipMarkSetResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipMarkSetRequest) -> GameResponse {
    info!("Handling EquipMarkSetRequest: {:?}", req);

    if let Some(equip_inven_index) = req.equip_inven_index {
        let _ = database::db::equip::equip_info::set_mark(
            pool,
            uid,
            equip_inven_index,
            req.mark.as_deref(),
        )
        .await;
    }

    let response = EquipMarkSetResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipMarkSet.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
