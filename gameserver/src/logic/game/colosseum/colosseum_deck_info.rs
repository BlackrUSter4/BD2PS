use bd2::prost::Message;
use bd2::proto::proto_net::{ColosseumDeckInfoRequest, ColosseumDeckInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::{build_deck_info_list, build_deck_item_info_list, default_notify};

pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumDeckInfoRequest) -> GameResponse {
    info!("Handling ColosseumDeckInfoRequest: {:?}", req);

    let response = ColosseumDeckInfoResponse {
        deck_info: build_deck_info_list(pool, uid).await,
        deck_item_info: build_deck_item_info_list(pool, uid).await,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumDeckInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
