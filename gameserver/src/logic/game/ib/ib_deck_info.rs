use bd2::prost::Message;
use bd2::proto::proto_net::{IbDeckInfoRequest, IbDeckInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::ib::ib_deck;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, deck_info_proto};

pub async fn handle(pool: &SqlitePool, uid: i64, req: IbDeckInfoRequest) -> GameResponse {
    info!("Handling IbDeckInfoRequest: {:?}", req);

    let rows = ib_deck::list(pool, uid).await.unwrap_or_default();
    let response = IbDeckInfoResponse { deck_info: deck_info_proto(&rows) };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::IbDeckInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
