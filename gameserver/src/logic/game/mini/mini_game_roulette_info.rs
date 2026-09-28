use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameRouletteDbInfo, MiniGameRouletteInfoRequest, MiniGameRouletteInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_roulette_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real per-account roulette progress, optionally scoped by the requested event_schedule_ids.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameRouletteInfoRequest) -> GameResponse {
    info!("Handling MiniGameRouletteInfoRequest: {:?}", req);

    let rows = db::get_mini_game_roulette_info(pool, uid).await.unwrap_or_default();
    let roulette_info = rows
        .into_iter()
        .filter(|r| req.event_schedule_id.is_empty() || r.event_schedule_id.map(|id| req.event_schedule_id.contains(&id)).unwrap_or(false))
        .map(|r| MiniGameRouletteDbInfo {
            event_schedule_id: r.event_schedule_id,
            free_ap_count: r.free_ap_count,
            reset_time: r.reset_time,
            is_reward_special_item: r.is_reward_special_item,
            try_count: r.try_count,
        })
        .collect();

    let response = MiniGameRouletteInfoResponse { roulette_info };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameRouletteInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
