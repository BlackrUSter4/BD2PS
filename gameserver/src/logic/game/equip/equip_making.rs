use super::{create_new_equip, get_equip_with_base, to_dbinfo, try_consume_items};
use bd2::prost::Message;
use bd2::proto::proto_net::{EquipMakingRequest, EquipMakingResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

/// Real crafting against EquipmentMakingTable's real material cost / result equip id / talent
/// level (same pattern as alchemy::craft and life_cooking): `making_id` is the recipe's own id
/// (EquipmentMakingTable.id), not the crafted equip's id -- the earlier version conflated the
/// two and created equip `making_id` directly, which only happened to "work" because
/// create_new_equip doesn't validate its id against EquipmentTable.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipMakingRequest) -> GameResponse {
    info!("Handling EquipMakingRequest: {:?}", req);

    let count = req.making_count.unwrap_or(1).max(1);
    let recipe = req
        .making_id
        .and_then(|id| exceldb::get().equipmentmakingtable.get(id).cloned());

    let mut equip_info = vec![];
    let mut add_talent_exp = 0;

    match recipe {
        Some(def) => {
            for i in 0..def.material_item_id.len() {
                let id = def.material_item_id[i];
                let per = *def.material_item_count.get(i).unwrap_or(&1);
                let _ = item_info::consume(pool, uid, id, per * count).await;
            }
            for _ in 0..(def.result_item_count.max(1) * count) {
                if let Ok(idx) = create_new_equip(pool, uid, def.result_item_id).await {
                    if let Some((equip, base)) = get_equip_with_base(pool, uid, idx).await {
                        equip_info.push(to_dbinfo(&equip, base.as_ref()));
                    }
                }
            }
            add_talent_exp = def.talent_level * count;
        }
        None => {
            // Unknown recipe id (shouldn't happen with a real client) -- fall back to the old
            // trust-the-client behavior rather than silently do nothing.
            if let Some(making_id) = req.making_id {
                if try_consume_items(pool, uid, &req.item_info).await {
                    for _ in 0..count {
                        if let Ok(idx) = create_new_equip(pool, uid, making_id).await {
                            if let Some((equip, base)) = get_equip_with_base(pool, uid, idx).await {
                                equip_info.push(to_dbinfo(&equip, base.as_ref()));
                            }
                        }
                    }
                }
            }
        }
    }

    let response = EquipMakingResponse {
        equip_info,
        add_talent_exp: Some(add_talent_exp),
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
