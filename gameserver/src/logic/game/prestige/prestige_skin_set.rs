use bd2::prost::Message;
use bd2::proto::proto_net::{PrestigeSkinSetRequest, PrestigeSkinSetResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{prestige::prestige_skin_info as db, user::user_info as user_db};
use sqlx::SqlitePool;
use tracing::info;

/// Real persistence into the account's real PrestigeSkinInfo table. Setting one as the active
/// skin (`is_set`) also updates the account's real portrait fields, since a "prestige skin" is
/// this project's special profile-portrait costume.
pub async fn handle(pool: &SqlitePool, uid: i64, req: PrestigeSkinSetRequest) -> GameResponse {
    info!("Handling PrestigeSkinSetRequest: {:?}", req);

    let mut portrait_costume_id = None;
    let mut portrait_costume_design_id = None;

    if let Some(skin) = &req.prestige_skin_info {
        if let Some(costume_id) = skin.costume_id {
            let is_set = skin.is_set.unwrap_or(false);
            let _ = db::upsert(pool, uid, costume_id, skin.costume_design_id, is_set).await;
            if is_set {
                let _ = user_db::update_portrait(pool, uid, costume_id, skin.costume_design_id).await;
                portrait_costume_id = Some(costume_id);
                portrait_costume_design_id = skin.costume_design_id;
            }
        }
    }

    let response = PrestigeSkinSetResponse { portrait_costume_id, portrait_costume_design_id };
    
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
    
    let (route, code) = PacketCodeType::PrestigeSkinSet.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}