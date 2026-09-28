use bd2::prost::Message;
use bd2::proto::proto_net::{IbItemInfoRequest, IbItemInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::ib::ib_inventory;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, item_info_proto};

pub async fn handle(pool: &SqlitePool, uid: i64, req: IbItemInfoRequest) -> GameResponse {
    info!("Handling IbItemInfoRequest: {:?}", req);

    let items = ib_inventory::list(pool, uid).await.unwrap_or_default();
    let response = IbItemInfoResponse { item_info: item_info_proto(&items) };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::IbItemInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
