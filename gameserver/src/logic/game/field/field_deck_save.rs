use bd2::prost::Message;
use bd2::proto::proto_net::{FieldDeckSaveRequest, FieldDeckSaveResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::field::field_deck_info as db;
use database::models::game::field::field_deck_info::FieldDeckInfo;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: FieldDeckSaveRequest) -> GameResponse {
    info!("Handling FieldDeckSaveRequest: {:?}", req);

    let items: Vec<FieldDeckInfo> = req
        .field_deck_info
        .iter()
        .map(|d| FieldDeckInfo {
            index: 0,
            uid,
            sequence: d.sequence,
            char_inven_index: d.char_inven_index,
            costume_inven_index: d.costume_inven_index,
        })
        .collect();

    if let Err(e) = db::replace_all(pool, uid, &items).await {
        tracing::error!("FieldDeckSave replace_all failed: {}", e);
    }

    let response = FieldDeckSaveResponse {};
    
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
    
    let (route, code) = PacketCodeType::FieldDeckSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}