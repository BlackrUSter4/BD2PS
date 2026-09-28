use bd2::prost::Message;
use bd2::proto::proto_net::{GachaSelectionSaveRequest, GachaSelectionSaveResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::gacha::gacha_selection_info::{add_gacha_selection_info, delete_gacha_selection_info};
use database::models::game::gacha::gacha_selection_info::GachaSelectionInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Full-replace save of the caller's gacha selection picks (which item a
/// "selection gacha" slot is currently set to) — real persistence.
pub async fn handle(pool: &SqlitePool, uid: i64, req: GachaSelectionSaveRequest) -> GameResponse {
    info!("Handling GachaSelectionSaveRequest: {:?}", req);

    if let Err(e) = delete_gacha_selection_info(pool, uid).await {
        tracing::warn!("GachaSelectionSave: failed to clear old selections: {}", e);
    }
    for s in &req.gacha_selection_info {
        let row = GachaSelectionInfo {
            index: 0,
            uid,
            group_id: s.group_id,
            slot: s.slot,
            item_id: s.item_id,
        };
        if let Err(e) = add_gacha_selection_info(pool, &row).await {
            tracing::warn!("GachaSelectionSave: failed to save selection: {}", e);
        }
    }

    let response = GachaSelectionSaveResponse {};
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

    let (route, code) = PacketCodeType::GachaSelectionSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
