use bd2::prost::Message;
use bd2::proto::proto_net::{FriendDbInfo, FriendSearchRequest, FriendSearchResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::user::user;
use sqlx::SqlitePool;
use tracing::info;

/// Real search: only returns a result if the target is a genuine real account on this
/// server. `user_id` is the target's real in-game nickname (see `logic::game::display_name`).
pub async fn handle(pool: &SqlitePool, _uid: i64, req: FriendSearchRequest) -> GameResponse {
    info!("Handling FriendSearchRequest: {:?}", req);

    let mut friend_info = None;
    if let Some(target) = req.owner_index {
        if user::find_account(pool, target).await.ok().flatten().is_some() {
            friend_info = Some(FriendDbInfo {
                owner_index: Some(target),
                user_id: Some(crate::logic::game::display_name(pool, target).await),
                ..Default::default()
            });
        }
    }

    let response = FriendSearchResponse { friend_info };
    
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
    
    let (route, code) = PacketCodeType::FriendSearch.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}