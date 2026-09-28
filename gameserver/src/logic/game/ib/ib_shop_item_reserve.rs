use bd2::prost::Message;
use bd2::proto::proto_net::{IbShopItemReserveRequest, IbShopItemReserveResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::ib::ib_shop;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(pool: &SqlitePool, uid: i64, req: IbShopItemReserveRequest) -> GameResponse {
    info!("Handling IbShopItemReserveRequest: {:?}", req);

    let slot = req.slot.unwrap_or(0);
    let is_reserved = req.is_reserved.unwrap_or(false);
    let _ = ib_shop::set_reserved(pool, uid, slot, is_reserved).await;

    let response = IbShopItemReserveResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::IbShopItemReserve.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
