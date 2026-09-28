use bd2::proto::proto_net::{AvatarShopWishListInfoRequest, AvatarShopWishListInfoResponse, Notify};
use bd2::prost::Message;
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::avatar::avatar_shop_wish_list_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: AvatarShopWishListInfoRequest) -> GameResponse {
    info!("Handling AvatarShopWishListInfoRequest: {:?}", req);

    let shop_id = avatar_shop_wish_list_info::get_all(pool, uid).await.unwrap_or_default();

    let response = AvatarShopWishListInfoResponse { shop_id };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::AvatarShopWishListInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
