use bd2::prost::Message;
use bd2::proto::proto_net::{JoinUserRequest, JoinUserResponse, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use crate::logic::game::account;

/// Real account bootstrap: reuses the same get-or-create-user path the REST LoginUser route
/// uses, returning the account's real UserDbInfo (freshly created on first contact).
pub async fn handle(pool: &SqlitePool, _uid: i64, req: JoinUserRequest) -> GameResponse {
    info!("Handling JoinUserRequest: {:?}", req);

    let access_token = req.access_token.as_deref().unwrap_or_default();
    let user_info = match account::parse_uid_from_token(access_token) {
        Ok(uid) => account::get_or_create_user(pool, uid).await.ok().map(|(_, u)| u.to_proto()),
        Err(_) => None,
    };

    let response = JoinUserResponse {
        user_info,
        reward_info_bundle: Some(RewardDbInfoBundle::default()),
    };

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

    let (route, code) = PacketCodeType::JoinUser.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
