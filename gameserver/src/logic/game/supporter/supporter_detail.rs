use bd2::prost::Message;
use bd2::proto::proto_net::{SupporterDetailRequest, SupporterDetailResponse, SupporterSlotInfo, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::supporter::supporter_slot_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real cross-account lookup of another (or one's own) account's registered slot.
pub async fn handle(pool: &SqlitePool, _uid: i64, req: SupporterDetailRequest) -> GameResponse {
    info!("Handling SupporterDetailRequest: {:?}", req);

    let mut supporter_info = None;
    if let (Some(target), Some(slot_index)) = (req.target_owner_index, req.slot_index) {
        if let Ok(Some(row)) = db::get_by_uid_and_slot(pool, target, slot_index).await {
            supporter_info = Some(SupporterSlotInfo {
                owner_index: Some(target),
                slot_index: row.slot_index,
                costume_id: row.costume_id,
                power: row.power,
                battle_use_count: row.battle_use_count,
                supporter_char_info: row.supporter_char_info,
                date: row.date,
            });
        }
    }

    let response = SupporterDetailResponse { supporter_info };
    
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
    
    let (route, code) = PacketCodeType::SupporterDetail.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}