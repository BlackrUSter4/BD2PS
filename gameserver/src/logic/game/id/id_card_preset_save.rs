use bd2::prost::Message;
use bd2::proto::proto_net::{IdCardPresetSaveRequest, IdCardPresetSaveResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::id::id_card_preset_info as preset_db;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: IdCardPresetSaveRequest) -> GameResponse {
    info!("Handling IdCardPresetSaveRequest: {:?}", req);

    if let Some(preset) = &req.preset_info {
        if let (Some(id), Some(card)) = (preset.id, &preset.id_card_info) {
            if let Some(row) = super::store_card_fresh(pool, uid, card).await {
                let _ = preset_db::upsert(pool, uid, id, row.index).await;
            }
        }
    }

    let response = IdCardPresetSaveResponse {};
    
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
    
    let (route, code) = PacketCodeType::IdCardPresetSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}