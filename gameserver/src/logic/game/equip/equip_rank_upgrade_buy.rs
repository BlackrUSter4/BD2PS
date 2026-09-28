use super::try_consume_items;
use bd2::prost::Message;
use bd2::proto::proto_net::{EquipRankUpgradeBuyRequest, EquipRankUpgradeBuyResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

const MAX_RANK: i32 = 4;

/// Same "buy a rank" mechanic as EquipSmelting, framed as a shop purchase (product_group_id/id)
/// rather than a raw materials sink — reuses the same rank-increment logic.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipRankUpgradeBuyRequest) -> GameResponse {
    info!("Handling EquipRankUpgradeBuyRequest: {:?}", req);

    let mut rank = vec![];
    if let Some(inven_index) = req.equip_inven_index {
        if let Some((_equip, Some(base))) =
            super::get_equip_with_base(pool, uid, inven_index).await
        {
            if base.rank < MAX_RANK && try_consume_items(pool, uid, &req.use_item_info).await {
                let new_rank = base.rank + 1;
                let _ =
                    database::db::equip::equip_base_info::set_rank(pool, uid, base.index, new_rank)
                        .await;
                rank = vec![new_rank];
            } else {
                rank = vec![base.rank];
            }
        }
    }

    let response = EquipRankUpgradeBuyResponse { rank, char_info: None };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipRankUpgradeBuy.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
