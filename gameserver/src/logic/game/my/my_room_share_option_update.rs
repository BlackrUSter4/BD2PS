use bd2::prost::Message;
use bd2::proto::proto_net::{MyRoomShareOptionUpdateRequest, MyRoomShareOptionUpdateResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::my::my_room_user_info;
use sqlx::SqlitePool;
use tracing::info;

use super::ensure_default_room;

pub async fn handle(pool: &SqlitePool, uid: i64, req: MyRoomShareOptionUpdateRequest) -> GameResponse {
    info!("Handling MyRoomShareOptionUpdateRequest: {:?}", req);

    ensure_default_room(pool, uid).await;
    if let Some(scope) = req.allow_scope_type {
        let _ = my_room_user_info::update_share_option(pool, uid, scope).await;
    }

    let response = MyRoomShareOptionUpdateResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::MyRoomShareOptionUpdate.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
