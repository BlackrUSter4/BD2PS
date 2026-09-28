use bd2::prost::Message;
use bd2::proto::proto_net::{TacticsBingoInfoRequest, TacticsBingoInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::tactics_bingo::tactics_bingo_clear_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// The request carries no `group_id` at all — treated as a single bingo card per event
/// (group_id 0), a judgment call with nothing in the schema to confirm or deny it.
pub async fn handle(pool: &SqlitePool, uid: i64, req: TacticsBingoInfoRequest) -> GameResponse {
    info!("Handling TacticsBingoInfoRequest: {:?}", req);

    let event_schedule_id = req.event_schedule_id.unwrap_or_default();
    let row = db::get(pool, uid, event_schedule_id, 0).await;
    let clear_stage = row
        .clear_stage
        .as_deref()
        .map(|s| s.split(',').filter_map(|p| p.parse().ok()).collect())
        .unwrap_or_default();

    let response = TacticsBingoInfoResponse {
        event_schedule_id: Some(event_schedule_id),
        group_id: Some(0),
        clear_stage,
        event_flag: row.event_flag,
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::TacticsBingoInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
