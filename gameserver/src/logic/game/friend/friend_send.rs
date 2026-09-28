use bd2::prost::Message;
use bd2::proto::proto_net::{FriendSendRequest, FriendSendResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{friend::friend_info as db, user::user};
use database::models::game::friend::friend_info::FriendInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Real friend-request send: mirrors a pending row on both sides (status=1 "I sent" on my
/// side, status=2 "they sent" on the target's side) — only if the target is a real account
/// and no relationship already exists either direction.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FriendSendRequest) -> GameResponse {
    info!("Handling FriendSendRequest: {:?}", req);

    if let Some(target) = req.owner_index {
        let target_is_real = user::find_account(pool, target).await.ok().flatten().is_some();
        let already_related = db::get_by_uid_and_owner(pool, uid, target).await.ok().flatten().is_some();

        if target_is_real && target != uid && !already_related {
            let now = chrono::Utc::now().timestamp_millis();
            let _ = db::add_friend_info(pool, &FriendInfo {
                index: 0, uid, owner_index: Some(target), portrait_costume_id: None,
                portrait_costume_design_id: None, user_id: None, title_id: None,
                date: Some(now), guild_base_info_index: None, last_login_date: None,
                supporter_info_index: None, status: 1,
            }).await;
            let _ = db::add_friend_info(pool, &FriendInfo {
                index: 0, uid: target, owner_index: Some(uid), portrait_costume_id: None,
                portrait_costume_design_id: None, user_id: None, title_id: None,
                date: Some(now), guild_base_info_index: None, last_login_date: None,
                supporter_info_index: None, status: 2,
            }).await;
        }
    }

    let response = FriendSendResponse {};
    
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
    
    let (route, code) = PacketCodeType::FriendSend.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}