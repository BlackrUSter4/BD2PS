use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameSurvivalPlayRequest, MiniGameSurvivalPlayResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Mid-run heartbeat — client reports the coin/exp gained since the last tick and the server
/// simply echoes them back as confirmed (same client-simulates/server-trusts pattern as every
/// other minigame in this project; no server-side combat resolution exists).
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: MiniGameSurvivalPlayRequest) -> GameResponse {
    info!("Handling MiniGameSurvivalPlayRequest: {:?}", req);

    let response = MiniGameSurvivalPlayResponse {
        server_increase_coin: req.increase_coin,
        server_increase_exp: req.increase_exp,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameSurvivalPlay.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
