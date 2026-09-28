use bd2::prost::Message;
use bd2::proto::proto_net::{CharImprintLevelUpRequest, CharImprintLevelUpResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::char::{char_awake_info, char_info};
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

/// Validates the target level against the character's real per-slot cap
/// from `CharAwakeTable` (imprintSlot1/2/3 — real, 17 rows captured).
pub async fn handle(pool: &SqlitePool, uid: i64, req: CharImprintLevelUpRequest) -> GameResponse {
    info!("Handling CharImprintLevelUpRequest: {:?}", req);

    let Some(char_inven_index) = req.char_inven_index else {
        return GameResponse::error(1);
    };
    let (Some(slot), Some(target_level)) = (req.slot, req.target_level) else {
        return GameResponse::error(1);
    };
    if !(1..=3).contains(&slot) {
        return GameResponse::error(1);
    }

    let Ok(Some(char_row)) = char_info::get_by_inven_index(pool, uid, char_inven_index).await
    else {
        return GameResponse::error(1);
    };
    let Some(unique_char_id) = char_row.id else {
        return GameResponse::error(1);
    };

    let game_data = exceldb::get();
    let cap = game_data
        .charawaketable
        .all()
        .iter()
        .find(|c| c.id == unique_char_id)
        .map(|c| match slot {
            1 => c.imprint_slot1,
            2 => c.imprint_slot2,
            _ => c.imprint_slot3,
        })
        .unwrap_or(0);

    if target_level > cap {
        return GameResponse::error(1);
    }

    let _ = char_awake_info::set_imprint_slot_level(pool, uid, unique_char_id, slot, target_level)
        .await;
    for item in &req.item_info {
        if let Some(idx) = item.inven_index {
            let _ = item_info::delete_by_inven_index(pool, uid, idx).await;
        }
    }

    let response = CharImprintLevelUpResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::CharImprintLevelUp.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
