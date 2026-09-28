use bd2::prost::Message;
use bd2::proto::proto_net::{IbShopInfoRequest, IbShopInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::ib::ib_shop;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, generate_shop_rows, shop_item_info_proto};

pub async fn handle(pool: &SqlitePool, uid: i64, req: IbShopInfoRequest) -> GameResponse {
    info!("Handling IbShopInfoRequest: {:?}", req);

    let mut rows = ib_shop::list(pool, uid).await.unwrap_or_default();
    if rows.is_empty() {
        let fresh = generate_shop_rows(uid);
        let _ = ib_shop::replace_all(pool, uid, &fresh).await;
        rows = fresh;
    }

    let response = IbShopInfoResponse { shop_item_info: shop_item_info_proto(&rows) };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::IbShopInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
