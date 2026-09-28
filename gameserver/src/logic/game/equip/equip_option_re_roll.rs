use super::{encode_option_list, get_equip_with_base, parse_option_list, try_consume_items};
use bd2::prost::Message;
use bd2::proto::proto_net::{EquipOptionInfo as ProtoOption, EquipOptionReRollRequest, EquipOptionReRollResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use rand::Rng;
use sqlx::SqlitePool;
use tracing::info;

/// Rerolls each unlocked option slot to a random id drawn from EquipmentTable's real
/// mainOptionGroupId/subOptionGroupId group(s) via EquipmentOptionTable (25 real rows) — locked
/// slots keep their current value. Writes the result immediately (see mod.rs's PrevXOptionIndex
/// stash) rather than holding a truly separate "pending" state in memory, since state has to
/// survive to a later, separate Confirm request anyway.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EquipOptionReRollRequest) -> GameResponse {
    info!("Handling EquipOptionReRollRequest: {:?}", req);

    let mut main_option = vec![];
    let mut sub_option = vec![];

    if let Some(inven_index) = req.equip_inven_index {
        if try_consume_items(pool, uid, &req.consume_item).await {
            if let Some((_equip, Some(base))) = get_equip_with_base(pool, uid, inven_index).await {
                let equip_def = base.id.and_then(|id| data::exceldb::get().equipmenttable.get(id));
                let prev_main = base.main_option_index.clone();
                let prev_sub = base.sub_option_index.clone();

                let cur_main = parse_option_list(&base.main_option_index);
                let cur_sub = parse_option_list(&base.sub_option_index);

                let main_groups = equip_def.map(|e| e.main_option_group_id.clone()).unwrap_or_default();
                let sub_groups = equip_def.map(|e| e.sub_option_group_id.clone()).unwrap_or_default();

                main_option = reroll_slots(&main_groups, &cur_main, &req.main_opt_lock);
                sub_option = reroll_slots(&sub_groups, &cur_sub, &req.sub_opt_lock);

                let _ = database::db::equip::equip_base_info::reroll_options(
                    pool,
                    uid,
                    base.index,
                    prev_main.as_deref(),
                    prev_sub.as_deref(),
                    encode_option_list(&main_option).as_deref(),
                    encode_option_list(&sub_option).as_deref(),
                )
                .await;
            }
        }
    }

    let response = EquipOptionReRollResponse { main_option, sub_option };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::EquipOptionReRoll.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}

fn reroll_slots(groups: &[i32], current: &[ProtoOption], locks: &[bool]) -> Vec<ProtoOption> {
    let option_table = &data::exceldb::get().equipmentoptiontable;
    groups
        .iter()
        .enumerate()
        .map(|(i, group_id)| {
            let locked = locks.get(i).copied().unwrap_or(false);
            if locked {
                if let Some(existing) = current.get(i) {
                    return existing.clone();
                }
            }
            let candidates: Vec<_> = option_table.by_group(*group_id).collect();
            let id = if candidates.is_empty() {
                0
            } else {
                let pick = rand::thread_rng().gen_range(0..candidates.len());
                candidates[pick].id
            };
            ProtoOption {
                group_id: Some(*group_id),
                id: Some(id),
            }
        })
        .collect()
}
