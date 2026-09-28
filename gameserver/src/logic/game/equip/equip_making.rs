use super::{create_new_equip, get_equip_with_base, to_dbinfo, try_consume_items};
use bd2::prost::Message;
use bd2::proto::proto_net::{EquipMakingRequest, EquipMakingResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipMakingRequest) -> GameResponse {
    info!("Handling EquipMakingRequest: {:?}", req);

    let mut equip_info = vec![];
    if let Some(making_id) = req.making_id {
        if try_consume_items(pool, uid, &req.item_info).await {
            let count = req.making_count.unwrap_or(1).max(1);
            for _ in 0..count {
                if let Ok(idx) = create_new_equip(pool, uid, making_id).await {
                    if let Some((equip, base)) = get_equip_with_base(pool, uid, idx).await {
                        equip_info.push(to_dbinfo(&equip, base.as_ref()));
                    }
                }
            }
        }
    }

    let response = EquipMakingResponse {
        equip_info,
        add_talent_exp: Some(0),
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipMaking.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
