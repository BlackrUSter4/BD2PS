use bd2::prost::Message;
use bd2::proto::proto_net::{SupporterBorrowRequest, SupporterBorrowResponse, SupporterDeckInfo, SupporterSlotInfo as ProtoSlot, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{friend::friend_info as friend_db, supporter::{supporter_slot_info as slot_db, supporter_usage_info as usage_db}};
use sqlx::SqlitePool;
use tracing::info;

/// Real borrow: records a real usage-history row and bumps the target slot's real
/// BattleUseCount. `supporter_char_info` is a real cross-account char+equip snapshot of the
/// *owning* account's registered character (see `super::build_battle_char_info`).
pub async fn handle(pool: &SqlitePool, uid: i64, req: SupporterBorrowRequest) -> GameResponse {
    info!("Handling SupporterBorrowRequest: {:?}", req);

    let mut supporter_info = None;
    let mut can_request_friend = false;
    let mut supporter_char_info = None;

    if let (Some(target), Some(slot_index)) = (req.supporter_owner_index, req.supporter_slot_index) {
        if let Ok(Some(row)) = slot_db::get_by_uid_and_slot(pool, target, slot_index).await {
            supporter_char_info =
                super::build_battle_char_info(pool, target, &row.supporter_char_info).await;
            let is_friend = friend_db::get_by_uid_and_owner(pool, uid, target).await.ok().flatten()
                .map(|f| f.status == 0).unwrap_or(false);
            can_request_friend = !is_friend && target != uid;

            let _ = slot_db::increment_use_count(pool, target, slot_index).await;
            let _ = usage_db::record_usage(
                pool, uid, uid, &uid.to_string(), target, slot_index, row.costume_id,
                if is_friend { 1 } else { 0 },
            ).await;

            supporter_info = Some(SupporterDeckInfo {
                owner_index: Some(target),
                user_id: Some(crate::logic::game::display_name(pool, target).await),
                title_id: None,
                portrait_costume_id: None,
                greeting: None,
                is_friend: Some(is_friend),
                guild_base_info: None,
                portrait_costume_design_id: None,
                usage_count: row.battle_use_count,
                supporter_slots: vec![ProtoSlot {
                    owner_index: Some(target),
                    slot_index: row.slot_index,
                    costume_id: row.costume_id,
                    power: row.power,
                    battle_use_count: row.battle_use_count,
                    supporter_char_info: row.supporter_char_info,
                    date: row.date,
                }],
            });
        }
    }

    let response = SupporterBorrowResponse {
        supporter_info,
        can_request_friend: Some(can_request_friend),
        supporter_char_info,
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
    
    let (route, code) = PacketCodeType::SupporterBorrow.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}