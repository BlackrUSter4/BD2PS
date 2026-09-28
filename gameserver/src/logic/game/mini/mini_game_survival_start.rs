use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameSurvivalStartRequest, MiniGameSurvivalStartResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// No per-session state needs to be created up front — the client resolves the run locally and
/// reports progress via MiniGameSurvivalPlay/End (same client-simulates/server-trusts pattern as
/// every other minigame). skill_info stays honestly empty — no starting-skill master table was
/// identified within this pass's scope; random_seed is server-generated so both sides agree on
/// spawn RNG.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: MiniGameSurvivalStartRequest) -> GameResponse {
    info!("Handling MiniGameSurvivalStartRequest: {:?}", req);

    let random_seed = (chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) & 0x7fffffff) as i32;

    let response = MiniGameSurvivalStartResponse { skill_info: vec![], random_seed: Some(random_seed) };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameSurvivalStart.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
