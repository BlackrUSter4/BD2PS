use bd2::prost::Message;
use bd2::proto::proto_net::{LikeUserRequest, LikeUserResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::my::my_like_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real like tracking (already-scaffolded MyLikeInfo table, just needed gameserver glue).
pub async fn handle(pool: &SqlitePool, uid: i64, req: LikeUserRequest) -> GameResponse {
    info!("Handling LikeUserRequest: {:?}", req);

    if let Some(target_owner_index) = req.target_owner_index {
        let now = chrono::Utc::now().timestamp_millis();
        let _ = db::upsert_like(pool, uid, target_owner_index, now).await;
    }

    let response = LikeUserResponse {};
    
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
    
    let (route, code) = PacketCodeType::LikeUser.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}