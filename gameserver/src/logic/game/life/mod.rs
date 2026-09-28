pub mod life_cheat_build_complete;
pub mod life_cheat_growth;
pub mod life_cheat_regen;
pub mod life_citizen_avatar_save;
pub mod life_citizen_recruit;
pub mod life_cooking;
pub mod life_crafting;
pub mod life_eat_food;
pub mod life_helper_assign;
pub mod life_helper_fire;
pub mod life_helper_gacha;
pub mod life_helper_gacha_delete;
pub mod life_helper_rename;
pub mod life_helper_reconnect;
pub mod life_helper_recruit;
pub mod life_helper_reward;
pub mod life_helper_reward_info;
pub mod life_info;
pub mod life_seeding;
pub mod life_shop_buy;
pub mod life_shop_buy_info;
pub mod life_shop_sell;
pub mod life_stat_save;
pub mod life_tool_upgrade;
pub mod life_user_info;
pub mod life_world_choice;
pub mod life_world_chunk_expand;
pub mod life_world_object_build_completed;
pub mod life_world_object_deco_save;
pub mod life_world_object_gathering;
pub mod life_world_object_place_save;
pub mod life_world_object_position_delta_save;
pub mod life_world_object_position_save;
pub mod life_world_object_status_save;
pub mod life_world_object_unplace_save;

use bd2::proto::proto_net::{
    AvatarUseDbInfo, ItemDbInfo, LifeCitizenDbInfo, LifeHelperDbInfo, LifeHelperGachaDbInfo,
    LifeUserDbInfo, LifeWorldObjectDbInfo, LifeWorldObjectPlaceDbInfo,
};
use database::models::game::item::item_info::ItemInfo;
use database::models::game::life::life_citizen_info::LifeCitizenInfo;
use database::models::game::life::life_helper_gacha_info::LifeHelperGachaInfo;
use database::models::game::life::life_helper_info::LifeHelperInfo;
use database::models::game::life::life_user_info::LifeUserInfo;
use database::models::game::life::life_world_object_info::LifeWorldObjectInfo;
use std::collections::BTreeMap;

pub fn now_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

/// Common (Uid, Id, Type, Count) triple pulled out of a client-sent ItemDbInfo — this is all
/// Life's item-consumption/reward logic needs; inventory-index bookkeeping is left to the
/// generic item system.
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
pub async fn try_consume_items(
    pool: &sqlx::SqlitePool,
    uid: i64,
    items: &[ItemDbInfo],
) -> bool {
    let deltas = item_deltas(items);
    for d in &deltas {
        match database::db::item::item_info::consume(pool, uid, d.id, d.count).await {
            Ok(true) => {}
            _ => return false,
        }
    }
    true
}

pub async fn grant_items(pool: &sqlx::SqlitePool, uid: i64, items: &[ItemDbInfo]) {
    for d in item_deltas(items) {
        let _ = database::db::item::item_info::grant(pool, uid, d.id, d.r#type, d.count).await;
    }
}

pub fn item_row_to_dbinfo(row: &ItemInfo) -> ItemDbInfo {
    ItemDbInfo {
        inven_index: row.inven_index,
        id: row.id,
        r#type: row.r#type,
        count: row.count,
        keep_flag: row.keep_flag,
        time_value: row.time_value,
        pictorialbook_info: None,
        expiry_time: row.expiry_time,
        sort_id: row.sort_id,
        use_count: row.use_count,
    }
}

pub fn avatar_dbinfo_from_fields(
    use_char_id: Option<i32>,
    use_hair_id: Option<i32>,
    use_hair_accessory_id: Option<i32>,
    use_face_accessory_id: Option<i32>,
    use_costume_id: Option<i32>,
    use_body_accessory_id: Option<i32>,
    use_hand_accessory_id: Option<i32>,
    use_pet_id: Option<i32>,
    use_mount_id: Option<i32>,
    use_effect_id: Option<i32>,
    date: Option<i64>,
) -> AvatarUseDbInfo {
    AvatarUseDbInfo {
        use_char_id,
        use_hair_id,
        use_hair_accessory_id,
        use_face_accessory_id,
        use_costume_id,
        use_body_accessory_id,
        use_hand_accessory_id,
        use_pet_id,
        use_mount_id,
        use_effect_id,
        date,
    }
}

pub fn citizen_row_to_dbinfo(row: &LifeCitizenInfo) -> LifeCitizenDbInfo {
    LifeCitizenDbInfo {
        citizen_index: row.citizen_index,
        citizen_slot_id: row.citizen_slot_id,
        avatar_info: Some(avatar_dbinfo_from_fields(
            row.use_char_id,
            row.use_hair_id,
            row.use_hair_accessory_id,
            row.use_face_accessory_id,
            row.use_costume_id,
            row.use_body_accessory_id,
            row.use_hand_accessory_id,
            row.use_pet_id,
            row.use_mount_id,
            row.use_effect_id,
            row.avatar_date,
        )),
    }
}

pub fn helper_row_to_dbinfo(row: &LifeHelperInfo) -> LifeHelperDbInfo {
    LifeHelperDbInfo {
        helper_index: row.helper_index,
        helper_id: row.helper_id,
        helper_slot_id: row.helper_slot_id,
        helper_name: row.helper_name.clone(),
        avatar_info: Some(avatar_dbinfo_from_fields(
            row.use_char_id,
            row.use_hair_id,
            row.use_hair_accessory_id,
            row.use_face_accessory_id,
            row.use_costume_id,
            row.use_body_accessory_id,
            row.use_hand_accessory_id,
            row.use_pet_id,
            row.use_mount_id,
            row.use_effect_id,
            row.avatar_date,
        )),
        work_type: row.work_type,
        work_id: row.work_id,
        assign_date: row.assign_date,
    }
}

pub fn helper_gacha_row_to_dbinfo(row: &LifeHelperGachaInfo) -> LifeHelperDbInfo {
    LifeHelperDbInfo {
        helper_index: None,
        helper_id: row.helper_id,
        helper_slot_id: row.helper_slot_id,
        helper_name: row.helper_name.clone(),
        avatar_info: Some(avatar_dbinfo_from_fields(
            row.use_char_id,
            row.use_hair_id,
            row.use_hair_accessory_id,
            row.use_face_accessory_id,
            row.use_costume_id,
            row.use_body_accessory_id,
            row.use_hand_accessory_id,
            row.use_pet_id,
            row.use_mount_id,
            row.use_effect_id,
            row.avatar_date,
        )),
        work_type: None,
        work_id: None,
        assign_date: None,
    }
}

pub fn helper_gacha_pool_to_dbinfo(
    helper_slot_id: i32,
    rows: &[LifeHelperGachaInfo],
) -> LifeHelperGachaDbInfo {
    LifeHelperGachaDbInfo {
        helper_slot_id: Some(helper_slot_id),
        gacha: rows.iter().map(helper_gacha_row_to_dbinfo).collect(),
    }
}

fn object_row_to_dbinfo(row: &LifeWorldObjectInfo, all: &[LifeWorldObjectInfo]) -> LifeWorldObjectDbInfo {
    let children: Vec<LifeWorldObjectDbInfo> = all
        .iter()
        .filter(|r| r.parent_index == Some(row.index))
        .map(|r| object_row_to_dbinfo(r, all))
        .collect();
    LifeWorldObjectDbInfo {
        x: row.x,
        y: row.y,
        rotate: row.rotate,
        index: row.object_index,
        object_id: row.object_id,
        status: row.status,
        start_time: row.start_time,
        end_time: row.end_time,
        inner_object: children,
    }
}

/// Group a flat set of placed-object rows (already scoped to one Uid) into
/// `LifeWorldObjectPlaceDbInfo` entries, one per chunk, with `inner_object` nesting
/// reconstructed from `ParentIndex`.
pub fn group_into_place_infos(rows: Vec<LifeWorldObjectInfo>) -> Vec<LifeWorldObjectPlaceDbInfo> {
    let mut by_chunk: BTreeMap<i32, Vec<LifeWorldObjectInfo>> = BTreeMap::new();
    for row in rows {
        by_chunk.entry(row.chunk_id).or_default().push(row);
    }
    by_chunk
        .into_iter()
        .map(|(chunk_id, objs)| LifeWorldObjectPlaceDbInfo {
            chunk_id: Some(chunk_id),
            object: objs
                .iter()
                .filter(|r| r.parent_index.is_none())
                .map(|r| object_row_to_dbinfo(r, &objs))
                .collect(),
        })
        .collect()
}

pub fn life_user_to_dbinfo(user: &LifeUserInfo, chunk_ids: Vec<i32>) -> LifeUserDbInfo {
    LifeUserDbInfo {
        life_coin: Some(user.life_coin),
        life_world_id: user.life_world_id,
        life_char_level_info: Some(bd2::proto::proto_net::LifeCharLevelDbInfo {
            logging_level: Some(user.logging_level),
            logging_exp: Some(user.logging_exp),
            mining_level: Some(user.mining_level),
            mining_exp: Some(user.mining_exp),
            farming_level: Some(user.farming_level),
            farming_exp: Some(user.farming_exp),
        }),
        chunk_id: chunk_ids,
    }
}

/// Flatten one incoming `LifeWorldObjectDbInfo` (ignoring any further-nested inner_object —
/// one level of nesting is all real placement flows are expected to send in a single save)
/// into a DB row for the given uid/chunk/parent.
pub fn place_dbinfo_to_row(
    uid: i64,
    chunk_id: i32,
    parent_index: Option<i64>,
    obj: &LifeWorldObjectDbInfo,
) -> LifeWorldObjectInfo {
    LifeWorldObjectInfo {
        index: 0,
        uid,
        chunk_id,
        object_index: obj.index,
        object_id: obj.object_id,
        x: obj.x,
        y: obj.y,
        rotate: obj.rotate,
        status: obj.status,
        start_time: obj.start_time,
        end_time: obj.end_time,
        parent_index,
    }
}
