use bd2::prost::Message;
use bd2::proto::proto_net::{DispatchRewardRequest, DispatchRewardResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::dispatch::dispatch_info as db;
use sqlx::SqlitePool;
use tracing::info;

use crate::logic::game::use2::roll_reward_group;

/// Real claim: only finished (real EndTime in the past) real DispatchInfo rows are claimable,
/// granting a real weighted roll from DispatchTable.reward_group_id via RewardGroupTable, then
/// removing the row.
pub async fn handle(pool: &SqlitePool, uid: i64, req: DispatchRewardRequest) -> GameResponse {
    info!("Handling DispatchRewardRequest: {:?}", req);

    let now = chrono::Utc::now().timestamp_millis();
    let game_data = data::exceldb::get();
    let mut item_info = Vec::new();

    for &id in &req.dispatch_id {
        let Some(row) = db::get_by_id(pool, uid, id).await.ok().flatten() else { continue };
        if row.end_time.unwrap_or(i64::MAX) > now {
            continue;
        }
        if let Some(def) = game_data.dispatchtable.get(id) {
            if let Some(reward) = roll_reward_group(pool, uid, def.reward_group_id).await {
                item_info.push(reward);
            }
        }
        let _ = db::delete_by_id(pool, uid, id).await;
    }

    let response = DispatchRewardResponse { item_info };

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

    let (route, code) = PacketCodeType::DispatchReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
