use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameHopscotchGameStartRequest, MiniGameHopscotchGameStartResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_hopscotch_play_state;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// MiniGameHopscotchGameEndRequest carries no stage/event reference of its own, so the in-flight
/// (event_schedule_id, stage_id) is tracked server-side here and consumed at GameEnd.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameHopscotchGameStartRequest) -> GameResponse {
    info!("Handling MiniGameHopscotchGameStartRequest: {:?}", req);

    let _ = mini_game_hopscotch_play_state::set(
        pool,
        uid,
        req.event_schedule_id.unwrap_or_default(),
        req.stage_id.unwrap_or_default(),
    )
    .await;

    let response = MiniGameHopscotchGameStartResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameHopscotchGameStart.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
