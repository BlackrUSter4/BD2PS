use bd2::prost::Message;
use bd2::proto::proto_net::{EquipsUseRequest, EquipsUseResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipsUseRequest) -> GameResponse {
    info!("Handling EquipsUseRequest: {:?}", req);

    if let Some(char_index) = req.char_index {
        for equip_index in &req.equip_index {
            let _ = database::db::equip::equip_info::set_use_char(
                pool,
                uid,
                *equip_index,
                Some(char_index),
            )
            .await;
        }
    }

    let response = EquipsUseResponse { char_info: None };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipsUse.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
