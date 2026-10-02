use bd2::proto::proto_net::{ItemDbInfo, Notify};
use database::db::item::item_info;
use sqlx::SqlitePool;

pub mod alchemy;
pub mod alchemy_batch;

pub fn default_notify() -> Notify {
    Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    }
}

/// Real crafting against AlchemyTable's real material cost / result item. `add_talent_exp` is
/// real TalentLevel/TalentExp progression for the crafting character (`inven_index`) via
/// `talent::add_talent_exp` — cross-checked against the reference server's
/// `GameTalentServer.Alchemy`, which credits the crafting character's own
/// TalentSkillTable.getExp per craft, not AlchemyTable's own `talentLevel` field (that field
/// isn't exp at all in the reference's own logic).
pub async fn craft(
    pool: &SqlitePool,
    uid: i64,
    alchemy_id: i32,
    count: i32,
    inven_index: Option<i64>,
) -> (Vec<ItemDbInfo>, i32) {
    let count = count.max(1);
    let Some(def) = data::exceldb::get().alchemytable.get(alchemy_id).cloned() else {
        return (vec![], 0);
    };

    for i in 0..def.material_item_id.len() {
        let id = def.material_item_id[i];
        let ty = *def.material_item_type.get(i).unwrap_or(&1);
        let per = *def.material_item_count.get(i).unwrap_or(&1);
        let _ = item_info::consume(pool, uid, id, per * count).await;
        let _ = ty;
    }

    let result_count = def.result_item_count.max(1) * count;
    let _ = item_info::grant(pool, uid, def.result_item_id, def.result_item_type, result_count).await;

    let item_info = vec![ItemDbInfo {
        id: Some(def.result_item_id),
        r#type: Some(def.result_item_type),
        count: Some(result_count),
        ..Default::default()
    }];

    let add_talent_exp = match inven_index {
        Some(idx) => super::talent::add_talent_exp(pool, uid, idx, count).await,
        None => 0,
    };

    (item_info, add_talent_exp)
}
