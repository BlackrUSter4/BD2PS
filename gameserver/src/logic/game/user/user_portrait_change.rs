use bd2::prost::Message;
use bd2::proto::proto_net::{UserPortraitChangeRequest, UserPortraitChangeResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::user::user_info as db;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: UserPortraitChangeRequest) -> GameResponse {
    info!("Handling UserPortraitChangeRequest: {:?}", req);

    let mut portrait_costume_id = None;
    if let Some(costume_id) = req.portrait_costume_id {
        let _ = db::update_portrait(pool, uid, costume_id, None).await;
        portrait_costume_id = Some(costume_id);
    }

    let response = UserPortraitChangeResponse { portrait_costume_id, portrait_costume_design_id: None };
    
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
    
    let (route, code) = PacketCodeType::UserPortraitChange.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}