use bd2::prost::Message;
use bd2::proto::proto_net::{AlchemyBatchRequest, AlchemyBatchResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::{craft, default_notify};

pub async fn handle(pool: &SqlitePool, uid: i64, req: AlchemyBatchRequest) -> GameResponse {
    info!("Handling AlchemyBatchRequest: {:?}", req);

    let (item_info, add_talent_exp) = match req.alchemy_id {
        Some(id) => craft(pool, uid, id, req.alchemy_count.unwrap_or(1), req.inven_index).await,
        None => (vec![], 0),
    };

    let response = AlchemyBatchResponse { item_info, add_talent_exp: Some(add_talent_exp) };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::AlchemyBatch.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
