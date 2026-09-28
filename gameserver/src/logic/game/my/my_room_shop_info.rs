use bd2::prost::Message;
use bd2::proto::proto_net::{MyRoomShopDbInfo, MyRoomShopInfoRequest, MyRoomShopInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::my::my_room_shop_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: MyRoomShopInfoRequest) -> GameResponse {
    info!("Handling MyRoomShopInfoRequest: {:?}", req);

    let shop_buy_info = my_room_shop_info::get_my_room_shop_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|row| MyRoomShopDbInfo {
            id: row.id,
            buy_count: row.buy_count,
        })
        .collect();

    let response = MyRoomShopInfoResponse { shop_buy_info };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::MyRoomShopInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
