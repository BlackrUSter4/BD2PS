use bd2::prost::Message;
use bd2::proto::proto_net::{EquipsClearRequest, EquipsClearResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipsClearRequest) -> GameResponse {
    info!("Handling EquipsClearRequest: {:?}", req);

    for clear in &req.clear_info {
        for equip_index in &clear.equip_inven_index {
            let _ = database::db::equip::equip_info::set_use_char(pool, uid, *equip_index, None)
                .await;
        }
    }

    let response = EquipsClearResponse { char_info: vec![] };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipsClear.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
