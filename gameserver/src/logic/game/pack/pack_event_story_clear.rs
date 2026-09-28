use bd2::prost::Message;
use bd2::proto::proto_net::{
    ItemDbInfo, PackEventStoryClearRequest, PackEventStoryClearResponse, PackEventStoryDbInfo,
    Notify, RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, pack::pack_event_story_info as db};
use sqlx::SqlitePool;
use tracing::info;

/// Real first-clear reward grant from PackEventStoryTable (284 real rows, real reward
/// arrays) — only pays out once per (group_id, id), tracked via a real per-account row.
pub async fn handle(pool: &SqlitePool, uid: i64, req: PackEventStoryClearRequest) -> GameResponse {
    info!("Handling PackEventStoryClearRequest: {:?}", req);

    let mut item_infos = Vec::new();
    if let (Some(event_uid), Some(group_id), Some(id)) = (req.event_uid, req.group_id, req.id) {
        if !db::is_cleared(pool, uid, group_id, id).await.unwrap_or(false) {
            if let Some(story) = data::exceldb::get().packeventstorytable.get(id) {
                for i in 0..story.reward_id.len() {
                    let rid = story.reward_id[i];
                    let ty = *story.reward_type.get(i).unwrap_or(&1);
                    let count = *story.reward_count.get(i).unwrap_or(&0);
                    if count > 0 {
                        let _ = item_info::grant(pool, uid, rid, ty, count).await;
                        item_infos.push(ItemDbInfo {
                            id: Some(rid),
                            r#type: Some(ty),
                            count: Some(count),
                            ..Default::default()
                        });
                    }
                }
            }
            let _ = db::mark_cleared(pool, uid, event_uid, group_id, id).await;
        }
    }

    let response = PackEventStoryClearResponse {
        reward_info_bundle: Some(RewardDbInfoBundle { item_info: item_infos, ..Default::default() }),
        clear_info: Some(PackEventStoryDbInfo {
            event_uid: req.event_uid,
            group_id: req.group_id,
            id: req.id,
        }),
    };
    
    let resp_bytes = response.encode_to_vec();
    
    let notify = Notify {
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
    };
    
    let (route, code) = PacketCodeType::PackEventStoryClear.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}