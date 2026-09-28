use bd2::prost::Message;
use bd2::proto::proto_net::{LoginUserRequest, LoginUserResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(_pool: &SqlitePool, _uid: i64, req: LoginUserRequest) -> GameResponse {
    info!("Handling LoginUserRequest: {:?}", req);
    
    // TODO: Fetch data from database
    // Example:
    // let data = get_something(pool, uid).await.unwrap_or_default();
    
    // TODO: Transform to proto
    // Example:
    // let proto_data = data.into_iter()
    //     .map(|item| mapper::to_proto(item, pool).await)
    //     .collect();
    
    let response = LoginUserResponse {
        // TODO: Fill in response fields
        ..Default::default()
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
    
    let (route, code) = PacketCodeType::LoginUser.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}