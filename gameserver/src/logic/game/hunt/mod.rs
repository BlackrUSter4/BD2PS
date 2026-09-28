pub mod hunt_dispatch;
pub mod hunt_dispatch_end;
pub mod hunt_dispatch_info;
pub mod hunt_dispatch_reward_preview;
pub mod hunt_dispatch_start;

use bd2::proto::proto_net::{ItemDbInfo, Notify, RewardDbInfoBundle};
use database::db::item::item_info as item_db;
use sqlx::SqlitePool;

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

/// Real reward computation from HuntDispatchTable's real visualItemId/visualItemType arrays,
/// scaled by the real rewardGrowthRate and the requested dispatch count. Grants for real.
pub async fn grant_dispatch_rewards(
    pool: &SqlitePool,
    uid: i64,
    dispatch: &data::exceldb::huntdispatchtable::Huntdispatchtable,
    count: i32,
) -> RewardDbInfoBundle {
    let rate = dispatch.reward_growth_rate.unwrap_or(1).max(1);
    let per_item = rate * count.max(1);

    let mut item_info = Vec::new();
    for i in 0..dispatch.visual_item_id.len() {
        let id = dispatch.visual_item_id[i];
        let ty = *dispatch.visual_item_type.get(i).unwrap_or(&1);
        let _ = item_db::grant(pool, uid, id, ty, per_item).await;
        item_info.push(ItemDbInfo { id: Some(id), r#type: Some(ty), count: Some(per_item), ..Default::default() });
    }

    RewardDbInfoBundle { item_info, ..Default::default() }
}
