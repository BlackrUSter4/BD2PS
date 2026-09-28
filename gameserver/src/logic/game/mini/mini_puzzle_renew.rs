use bd2::prost::Message;
use bd2::proto::proto_net::{MiniPuzzleRenewRequest, MiniPuzzleRenewResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_puzzle_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::mini_puzzle_info::to_proto;

/// Real puzzle reset: clears the real PuzzleOpen bitmask for a fresh board.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniPuzzleRenewRequest) -> GameResponse {
    info!("Handling MiniPuzzleRenewRequest: {:?}", req);

    let mut before_puzzle_info = None;
    let mut mini_puzzle_info = None;

    if let Some(event_schedule_id) = req.event_schedule_id {
        if let Some(row) = db::get_or_create(pool, uid, event_schedule_id).await.ok() {
            before_puzzle_info = Some(to_proto(&row));
            let _ = db::update_open(pool, row.index, 0, row.clear_count.unwrap_or(0)).await;
            let mut updated = row;
            updated.puzzle_open = 0;
            mini_puzzle_info = Some(to_proto(&updated));
        }
    }

    let response = MiniPuzzleRenewResponse { before_puzzle_info, mini_puzzle_info };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::MiniPuzzleRenew.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
