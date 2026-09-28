use bd2::prost::Message;
use bd2::proto::proto_net::{MyRoomItemInfoRequest, MyRoomItemInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::build_trophy_list;

pub async fn handle(pool: &SqlitePool, uid: i64, req: MyRoomItemInfoRequest) -> GameResponse {
    info!("Handling MyRoomItemInfoRequest: {:?}", req);

    let response = MyRoomItemInfoResponse {
        trophy_info: build_trophy_list(pool, uid).await,
    };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::MyRoomItemInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
