use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameBoardDbInfo, MiniGameBoardInfoRequest, MiniGameBoardInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_board_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real per-account board progress, optionally scoped by the requested event_schedule_ids.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameBoardInfoRequest) -> GameResponse {
    info!("Handling MiniGameBoardInfoRequest: {:?}", req);

    let rows = db::get_mini_game_board_info(pool, uid).await.unwrap_or_default();
    let mini_game_board_info = rows
        .into_iter()
        .filter(|r| req.event_schedule_id.is_empty() || r.event_schedule_id.map(|id| req.event_schedule_id.contains(&id)).unwrap_or(false))
        .map(|r| MiniGameBoardDbInfo {
            event_schedule_id: r.event_schedule_id,
            scaffold_group_id: r.scaffold_group_id,
            scaffold_id: r.scaffold_id,
            complete_count: r.complete_count,
        })
        .collect();

    let response = MiniGameBoardInfoResponse { mini_game_board_info };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameBoardInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
