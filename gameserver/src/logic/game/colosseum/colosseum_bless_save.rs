use bd2::prost::Message;
use bd2::proto::proto_net::{ColosseumBlessSaveRequest, ColosseumBlessSaveResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::colosseum::colosseum_bless_info;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumBlessSaveRequest) -> GameResponse {
    info!("Handling ColosseumBlessSaveRequest: {:?}", req);

    for entry in &req.bless_info {
        let deck_type = entry.deck_type.unwrap_or(0);
        let _ = colosseum_bless_info::replace_for_type(pool, uid, deck_type, &entry.id).await;
    }

    let response = ColosseumBlessSaveResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumBlessSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
