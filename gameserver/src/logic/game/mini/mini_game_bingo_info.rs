use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameBingoDbInfo, MiniGameBingoInfoRequest, MiniGameBingoInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_bingo_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

fn parse_list(s: &str) -> Vec<i32> {
    serde_json::from_str(s).unwrap_or_default()
}

/// Real per-account bingo progress, optionally scoped by the requested event_schedule_ids.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameBingoInfoRequest) -> GameResponse {
    info!("Handling MiniGameBingoInfoRequest: {:?}", req);

    let rows = db::get_mini_game_bingo_info(pool, uid).await.unwrap_or_default();
    let mini_game_bingo_info = rows
        .into_iter()
        .filter(|r| req.event_schedule_id.is_empty() || r.event_schedule_id.map(|id| req.event_schedule_id.contains(&id)).unwrap_or(false))
        .map(|r| MiniGameBingoDbInfo {
            event_schedule_id: r.event_schedule_id,
            clear_count: r.clear_count,
            bingo_board: parse_list(&r.bingo_board),
            open_bingo_board_index: parse_list(&r.open_bingo_board_index),
        })
        .collect();

    let response = MiniGameBingoInfoResponse { mini_game_bingo_info };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameBingoInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
