use bd2::prost::Message;
use bd2::proto::proto_net::{IbDungeonGiveUpRequest, IbDungeonGiveUpResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::ib::ib_play_state;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(pool: &SqlitePool, uid: i64, req: IbDungeonGiveUpRequest) -> GameResponse {
    info!("Handling IbDungeonGiveUpRequest: {:?}", req);

    let _ = ib_play_state::give_up(pool, uid).await;

    let response = IbDungeonGiveUpResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::IbDungeonGiveUp.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
