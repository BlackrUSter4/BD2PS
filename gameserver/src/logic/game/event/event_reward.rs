use bd2::prost::Message;
use bd2::proto::proto_net::{EventRewardRequest, EventRewardResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{event::event_reward_history_info as history_db, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

const GOLD_ITEM_ID: i32 = 4;
const GOLD_ITEM_TYPE: i32 = 1;

/// Real claim-once tracking via EventRewardHistoryInfo. No generic "event schedule reward"
/// master table was captured for any event (only EventMissionCompleteRewardTable exists, and
/// that belongs to the separate EventMission cluster), so like the MonsterHunt/Pack rounds a
/// fixed gold placeholder is granted the first time a given (schedule, group, reward) triple
/// is claimed, and refused as a duplicate on any repeat claim.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EventRewardRequest) -> GameResponse {
    info!("Handling EventRewardRequest: {:?}", req);

    let event_schedule_id = req.event_schedule_id.unwrap_or(0);
    let event_group_id = req.reward_group_id.unwrap_or(0);
    let reward_id = req.reward_id.unwrap_or(0);

    let already = history_db::has_claimed(pool, uid, event_schedule_id, event_group_id, reward_id)
        .await
        .unwrap_or(false);

    if !already {
        let _ = item_info::grant(pool, uid, GOLD_ITEM_ID, GOLD_ITEM_TYPE, 1000).await;
        let _ = history_db::mark_claimed(pool, uid, event_schedule_id, event_group_id, reward_id).await;
    }

    let response = EventRewardResponse {};

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

    let (route, code) = PacketCodeType::EventReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
