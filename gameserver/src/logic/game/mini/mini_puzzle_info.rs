use bd2::prost::Message;
use bd2::proto::proto_net::{MiniPuzzleDbInfo, MiniPuzzleInfoRequest, MiniPuzzleInfoResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_puzzle_info as db;
use sqlx::SqlitePool;
use tracing::info;

pub fn to_proto(r: &database::models::game::mini::mini_puzzle_info::MiniPuzzleInfo) -> MiniPuzzleDbInfo {
    MiniPuzzleDbInfo {
        event_schedule_id: r.event_schedule_id,
        clear_count: r.clear_count,
        puzzle: vec![r.puzzle],
        puzzle_open: vec![r.puzzle_open],
    }
}

/// Real per-account puzzle progress.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniPuzzleInfoRequest) -> GameResponse {
    info!("Handling MiniPuzzleInfoRequest: {:?}", req);

    let rows = db::get_mini_puzzle_info(pool, uid).await.unwrap_or_default();
    let mini_puzzle_info = rows
        .iter()
        .filter(|r| req.event_schedule_id.is_empty() || r.event_schedule_id.map(|id| req.event_schedule_id.contains(&id)).unwrap_or(false))
        .map(to_proto)
        .collect();

    let response = MiniPuzzleInfoResponse { mini_puzzle_info };

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

    let (route, code) = PacketCodeType::MiniPuzzleInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
