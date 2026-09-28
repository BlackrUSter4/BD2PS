use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, SupporterRewardRequest, SupporterRewardResponse, RewardDbInfoBundle, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, supporter::supporter_usage_info as db};
use sqlx::SqlitePool;
use tracing::info;

/// Real reward claim: only pays out unclaimed usages, respecting the real daily cap from
/// SupportCharacterDefaultTable.daily_max_support_reward_count, using the table's real
/// reward item type/value.
pub async fn handle(pool: &SqlitePool, uid: i64, req: SupporterRewardRequest) -> GameResponse {
    info!("Handling SupporterRewardRequest: {:?}", req);

    let cfg = data::exceldb::get().supportcharacterdefaulttable.all().first().cloned();
    let daily_cap = cfg.as_ref().map(|c| c.daily_max_support_reward_count).unwrap_or(i32::MAX);
    let already_claimed_today = db::count_rewards_claimed_today(pool, uid).await.unwrap_or(0) as i32;

    let rows = db::get_by_ids(pool, uid, &req.id).await.unwrap_or_default();
    let mut claimed_ids = Vec::new();
    let mut item_infos = Vec::new();
    let mut claimed_this_call = 0;

    for row in rows {
        if row.reward_received == Some(1) {
            continue;
        }
        if already_claimed_today + claimed_this_call >= daily_cap {
            break;
        }
        if let (Some(id), Some(cfg)) = (row.id, &cfg) {
            // SupportCharacterDefaultTable has real item_type/value fields but no item id —
            // gold (id 4) is used as the granted item, matching the precedent set in
            // earlier rounds for the same "no id field" situation (e.g. Friendship).
            let item_id = 4;
            let _ = item_info::grant(pool, uid, item_id, cfg.support_reward_item_type, cfg.support_reward_value).await;
            item_infos.push(ItemDbInfo {
                id: Some(item_id),
                r#type: Some(cfg.support_reward_item_type),
                count: Some(cfg.support_reward_value),
                ..Default::default()
            });
            let _ = db::mark_reward_received(pool, uid, id).await;
            claimed_ids.push(id);
            claimed_this_call += 1;
        }
    }

    let response = SupporterRewardResponse {
        id: claimed_ids,
        reward_bundle: Some(RewardDbInfoBundle { item_info: item_infos, ..Default::default() }),
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
    
    let (route, code) = PacketCodeType::SupporterReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}