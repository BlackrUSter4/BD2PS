use bd2::prost::Message;
use bd2::proto::proto_net::{CashBonusRewardRequest, CashBonusRewardResponse, ItemDbInfo, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{cash::cash_bonus_info as db, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

/// No CashBonusTable master data exists anywhere — buy_count and rewarded-id tracking are
/// real, but the reward itself is a flat placeholder grant.
const REWARD_ITEM_ID: i32 = 4;
const REWARD_ITEM_TYPE: i32 = 1;
const REWARD_COUNT: i32 = 50;

pub async fn handle(pool: &SqlitePool, uid: i64, req: CashBonusRewardRequest) -> GameResponse {
    info!("Handling CashBonusRewardRequest: {:?}", req);

    let group_id = req.group_id.unwrap_or_default();
    let contents_group_id = req.contents_group_id.unwrap_or_default();
    let bonus_id = req.bonus_id.unwrap_or_default();

    let mut row = db::get_one(pool, uid, group_id, contents_group_id).await;
    let mut rewarded: Vec<i32> = row
        .rewarded_ids
        .as_deref()
        .map(|s| s.split(',').filter_map(|p| p.parse().ok()).collect())
        .unwrap_or_default();

    let already = rewarded.contains(&bonus_id);
    let reward_info_bundle = if already {
        RewardDbInfoBundle::default()
    } else {
        rewarded.push(bonus_id);
        row.buy_count += 1;
        row.rewarded_ids = Some(rewarded.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(","));
        let _ = db::save(pool, &row).await;

        let _ = item_info::grant(pool, uid, REWARD_ITEM_ID, REWARD_ITEM_TYPE, REWARD_COUNT).await;
        RewardDbInfoBundle {
            item_info: vec![ItemDbInfo {
                id: Some(REWARD_ITEM_ID),
                r#type: Some(REWARD_ITEM_TYPE),
                count: Some(REWARD_COUNT),
                ..Default::default()
            }],
            ..Default::default()
        }
    };

    let response = CashBonusRewardResponse {
        reward_info_bundle: Some(reward_info_bundle),
        rewarded_id: rewarded,
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
    let (route, code) = PacketCodeType::CashBonusReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
