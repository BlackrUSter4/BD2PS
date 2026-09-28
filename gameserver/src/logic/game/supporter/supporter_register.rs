use bd2::prost::Message;
use bd2::proto::proto_net::{SupporterRegisterRequest, SupporterRegisterResponse, SupporterSlotInfo, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::supporter::supporter_slot_info as db;
use serde::Serialize;
use sqlx::SqlitePool;
use tracing::info;

#[derive(Serialize)]
struct CharRef {
    char_inven_index: Option<i64>,
    costume_inven_index: Option<i64>,
}

/// Real slot registration. `costume_id` in the response is left as the request's own
/// costume_inven_index cast down — no lookup from inventory to a real costume definition
/// id exists in scope here, so the raw inven index is echoed rather than fabricated.
pub async fn handle(pool: &SqlitePool, uid: i64, req: SupporterRegisterRequest) -> GameResponse {
    info!("Handling SupporterRegisterRequest: {:?}", req);

    let mut supporter_slot = None;
    if let Some(slot_index) = req.slot_index {
        let char_info = serde_json::to_string(&CharRef {
            char_inven_index: req.char_inven_index,
            costume_inven_index: req.costume_inven_index,
        }).ok();
        let costume_id = req.costume_inven_index.map(|c| c as i32);
        let _ = db::upsert(pool, uid, slot_index, costume_id, req.power, char_info.as_deref()).await;

        if let Ok(Some(row)) = db::get_by_uid_and_slot(pool, uid, slot_index).await {
            supporter_slot = Some(SupporterSlotInfo {
                owner_index: Some(uid),
                slot_index: row.slot_index,
                costume_id: row.costume_id,
                power: row.power,
                battle_use_count: row.battle_use_count,
                supporter_char_info: row.supporter_char_info,
                date: row.date,
            });
        }
    }

    let response = SupporterRegisterResponse { supporter_slot };
    
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
    
    let (route, code) = PacketCodeType::SupporterRegister.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}