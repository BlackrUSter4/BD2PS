use bd2::prost::Message;
use bd2::proto::proto_net::{
    ItemDbInfo, Notify, PackEventStoryReplayClearRequest, PackEventStoryReplayClearResponse,
    RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

/// Judgment call: a "replay" clear re-grants the same real PackEventStoryTable reward
/// every time (no separate replay-reward table exists) rather than being gated on
/// first-clear like PackEventStoryClear — that's the whole point of a replay request.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: PackEventStoryReplayClearRequest,
) -> GameResponse {
    info!("Handling PackEventStoryReplayClearRequest: {:?}", req);

    let mut item_infos = Vec::new();
    if let Some(id) = req.id {
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
    }

    let response = PackEventStoryReplayClearResponse {
        reward_info_bundle: Some(RewardDbInfoBundle { item_info: item_infos, ..Default::default() }),
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

    let (route, code) = PacketCodeType::PackEventStoryReplayClear.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
