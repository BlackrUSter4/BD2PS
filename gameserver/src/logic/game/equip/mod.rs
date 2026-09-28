pub mod equip_main_opt_change;
pub mod equip_making_to_break_auto;
pub mod equip_rank_upgrade_buy;
pub mod equip_upgrade_to_break_auto;

pub mod equip_add_slot;
pub mod equip_batch_use;
pub mod equip_break;
pub mod equip_change;
pub mod equip_clear;
pub mod equip_from_making_to_break;
pub mod equip_from_making_to_upgrade;
pub mod equip_from_upgrade_to_break;
pub mod equip_info;
pub mod equip_lock;
pub mod equip_making;
pub mod equip_mark_delete;
pub mod equip_mark_set;
pub mod equip_option_re_roll;
pub mod equip_option_re_roll_confirm;
pub mod equip_preset_info;
pub mod equip_preset_name_change;
pub mod equip_preset_save;
pub mod equip_sequence_smelting;
pub mod equip_sequence_upgrade;
pub mod equip_smelting;
pub mod equip_storage_add_slot;
pub mod equip_storage_info;
pub mod equip_storage_out;
pub mod equip_storage_put;
pub mod equip_upgrade;
pub mod equip_use;

use bd2::proto::proto_net::{EquipBaseInfo as ProtoEquipBaseInfo, EquipDbInfo, EquipOptionInfo as ProtoEquipOptionInfo, ItemDbInfo};
use database::db::item::item_info;
use database::models::game::equip::equip_base_info::EquipBaseInfo;
use database::models::game::equip::equip_info::EquipInfo;
use sqlx::SqlitePool;

/// Common (id, type, count) triple pulled out of a client-sent ItemDbInfo — matches the
/// convention already established in life/fishing/my_room/avatar's own copies of this helper.
pub struct ItemDelta {
    pub id: i32,
    pub r#type: i32,
    pub count: i32,
}

pub fn item_deltas(items: &[ItemDbInfo]) -> Vec<ItemDelta> {
    items
        .iter()
        .filter_map(|i| {
            Some(ItemDelta {
                id: i.id?,
                r#type: i.r#type.unwrap_or(0),
                count: i.count.unwrap_or(1),
            })
        })
        .collect()
}

/// Try to consume every listed item; returns false (consuming nothing further) the moment one
/// can't be afforded, so a request never partially consumes materials it can't fully pay for.
pub async fn try_consume_items(pool: &SqlitePool, uid: i64, items: &[ItemDbInfo]) -> bool {
    let deltas = item_deltas(items);
    for d in &deltas {
        match item_info::consume(pool, uid, d.id, d.count).await {
            Ok(true) => {}
            _ => return false,
        }
    }
    true
}

pub async fn grant_items(pool: &SqlitePool, uid: i64, items: &[ItemDbInfo]) {
    for d in item_deltas(items) {
        let _ = item_info::grant(pool, uid, d.id, d.r#type, d.count).await;
    }
}

/// Parses a JSON-array-of-(group_id,id)-pairs TEXT column into proto EquipOptionInfo list.
/// This project has no fixed convention for serializing repeated option pairs into a single
/// TEXT column, so a compact JSON array of [group_id, id] tuples was chosen for this round.
pub fn parse_option_list(text: &Option<String>) -> Vec<ProtoEquipOptionInfo> {
    let Some(text) = text else { return vec![] };
    let pairs: Vec<(i32, i32)> = serde_json::from_str(text).unwrap_or_default();
    pairs
        .into_iter()
        .map(|(group_id, id)| ProtoEquipOptionInfo {
            group_id: Some(group_id),
            id: Some(id),
        })
        .collect()
}

pub fn encode_option_list(options: &[ProtoEquipOptionInfo]) -> Option<String> {
    let pairs: Vec<(i32, i32)> = options
        .iter()
        .map(|o| (o.group_id.unwrap_or(0), o.id.unwrap_or(0)))
        .collect();
    if pairs.is_empty() {
        None
    } else {
        Some(serde_json::to_string(&pairs).unwrap_or_default())
    }
}

/// Builds a client-facing EquipDbInfo from an EquipInfo row plus its joined EquipBaseInfo row
/// (if the base row is missing — shouldn't normally happen — falls back to an empty base_info).
pub fn to_dbinfo(equip: &EquipInfo, base: Option<&EquipBaseInfo>) -> EquipDbInfo {
    let base_info = base.map(|b| ProtoEquipBaseInfo {
        id: b.id,
        level: b.level,
        main_option: parse_option_list(&b.main_option_index),
        sub_option: parse_option_list(&b.sub_option_index),
        private_option: None,
        rank: vec![b.rank],
    });

    EquipDbInfo {
        inven_index: equip.inven_index,
        use_char: equip.use_char,
        keep_flag: equip.keep_flag,
        lock_flag: equip.lock_flag,
        base_info,
        pictorialbook_info: None,
        sort_id: None,
        mark: equip.mark.clone(),
    }
}

/// Fetches an equip row + its joined base row together, for handlers that need both.
pub async fn get_equip_with_base(
    pool: &SqlitePool,
    uid: i64,
    inven_index: i64,
) -> Option<(EquipInfo, Option<EquipBaseInfo>)> {
    let equip = database::db::equip::equip_info::get_by_inven_index(pool, uid, inven_index)
        .await
        .ok()??;
    let base = match equip.base_info_index {
        Some(idx) => database::db::equip::equip_base_info::get_by_index(pool, uid, idx)
            .await
            .ok()
            .flatten(),
        None => None,
    };
    Some((equip, base))
}

/// Creates a brand-new equip (fresh EquipBaseInfo at level 1 rank 1 + its owning EquipInfo row),
/// stamping InvenIndex so the client has a stable id to reference it by afterward.
pub async fn create_new_equip(pool: &SqlitePool, uid: i64, equip_id: i32) -> sqlx::Result<i64> {
    let base_index = database::db::equip::equip_base_info::insert(
        pool,
        &EquipBaseInfo {
            index: 0,
            uid,
            id: Some(equip_id),
            level: Some(1),
            main_option_index: None,
            sub_option_index: None,
            private_option_index: None,
            rank: 1,
            prev_main_option_index: None,
            prev_sub_option_index: None,
        },
    )
    .await?;

    let equip_index = database::db::equip::equip_info::insert(
        pool,
        &EquipInfo {
            index: 0,
            uid,
            inven_index: None,
            use_char: None,
            keep_flag: Some(0),
            lock_flag: Some(0),
            base_info_index: Some(base_index),
            mark: None,
        },
    )
    .await?;
    database::db::equip::equip_info::set_inven_index_to_own_index(pool, equip_index).await?;
    Ok(equip_index)
}

/// Shared "repeatedly upgrade toward a target level" loop used by the Sequence/Auto/FromMaking
/// combo requests. No EquipmentGrowthTable row matches any captured equip id (see equip_break's
/// note), so there's no real success-ratio/gold-cost data to simulate failure with — every
/// attempt succeeds until the target or the item's real max level (from EquipmentTable) is hit.
/// Returns (final_level, try_count).
pub async fn auto_upgrade_to_level(
    pool: &SqlitePool,
    uid: i64,
    base: &mut EquipBaseInfo,
    target_level: i32,
) -> (i32, i32) {
    let equip_id = base.id.unwrap_or(0);
    let max_level = max_level_for(equip_id);
    let cap = target_level.min(max_level);
    let mut level = base.level.unwrap_or(1);
    let mut try_count = 0;
    while level < cap {
        level += 1;
        try_count += 1;
    }
    if try_count > 0 {
        let _ = database::db::equip::equip_base_info::set_level(pool, uid, base.index, level).await;
        base.level = Some(level);
    }
    (level, try_count)
}

/// Shared JSON-array-of-i64 codec, used for the several "list of child row indices" TEXT
/// columns in the preset tables (same TEXT-list convention as `parse_option_list` above).
pub fn parse_index_list(text: &Option<String>) -> Vec<i64> {
    text.as_ref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or_default()
}

pub fn encode_index_list(indices: &[i64]) -> Option<String> {
    if indices.is_empty() {
        None
    } else {
        Some(serde_json::to_string(indices).unwrap_or_default())
    }
}

/// Look up an equip id's max level from EquipmentTable master data (falls back to a generous
/// placeholder cap if the id isn't in the (sparsely-captured) table at all).
pub fn max_level_for(equip_id: i32) -> i32 {
    data::exceldb::get()
        .equipmenttable
        .get(equip_id)
        .map(|e| e.max_level)
        .unwrap_or(20)
}
