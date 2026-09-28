use bd2::prost::Message;
use bd2::proto::proto_net::{CostumeBaseDbInfo, SupporterStatusRequest, SupporterStatusResponse, SupporterUsageInfo as ProtoUsage, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::supporter::{supporter_slot_info as slot_db, supporter_usage_info as usage_db};
use sqlx::SqlitePool;
use tracing::info;

/// Real rental history + own registered slots + real daily-claim count.
/// `guild_supporter_info` is left empty (separate guild-scoped table, out of scope here).
pub async fn handle(pool: &SqlitePool, uid: i64, req: SupporterStatusRequest) -> GameResponse {
    info!("Handling SupporterStatusRequest: {:?}", req);

    let limit = req.limit.unwrap_or(20);
    let history = usage_db::get_history(pool, uid, req.last_id, limit).await.unwrap_or_default();
    let supporter_rental_history = history
        .into_iter()
        .map(|r| ProtoUsage {
            id: r.id,
            borrower_owner_index: r.borrower_owner_index,
            borrower_user_id: r.borrower_user_id,
            borrower_portrait_costume_id: r.borrower_portrait_costume_id,
            borrower_portrait_design_id: r.borrower_portrait_design_id,
            borrower_title_id: r.borrower_title_id,
            supporter_owner_index: r.supporter_owner_index,
            supporter_slot_index: r.supporter_slot_index,
            supporter_costume_id: r.supporter_costume_id,
            supporter_design_id: r.supporter_design_id,
            borrow_type: r.borrow_type,
            reward_received: r.reward_received.map(|v| v != 0),
            use_date: r.use_date,
        })
        .collect();

    let slots = slot_db::get_supporter_slot_info(pool, uid).await.unwrap_or_default();
    let supporter_info = slots
        .into_iter()
        .filter_map(|r| r.costume_id.map(|id| CostumeBaseDbInfo { id: Some(id), level: None, design_id: None }))
        .collect();

    let daily_reward_claim_count = usage_db::count_rewards_claimed_today(pool, uid).await.unwrap_or(0) as i32;

    let response = SupporterStatusResponse {
        supporter_rental_history,
        supporter_info,
        guild_supporter_info: vec![],
        daily_reward_claim_count: Some(daily_reward_claim_count),
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
    
    let (route, code) = PacketCodeType::SupporterStatus.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}