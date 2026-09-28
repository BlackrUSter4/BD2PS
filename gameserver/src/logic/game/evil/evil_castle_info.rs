use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleDbInfo, EvilCastleInfoRequest, EvilCastleInfoResponse, EvilCastleTotalDbInfo, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::{evil_castle_info, evil_castle_total_info};
use sqlx::SqlitePool;
use tracing::info;

/// Single fixed season for now — no season-schedule table exists anywhere
/// in the client's own schema for EvilCastle, so there's nothing to derive
/// rollover timing from (same "one always-active season" call already made
/// for CharVote/Colosseum).
pub const CURRENT_SEASON: i32 = 1;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleInfoRequest) -> GameResponse {
    info!("Handling EvilCastleInfoRequest: {:?}", req);

    let rows = evil_castle_info::get_evil_castle_info(pool, uid)
        .await
        .unwrap_or_default();
    let evil_castle_info_list = rows
        .into_iter()
        .map(|r| EvilCastleDbInfo {
            rank: None,
            stage_index: r.stage_index,
            retry: r.retry,
            point: r.point,
            season_highest_point: r.season_highest_point,
            is_rewarded: r.is_rewarded,
            stage_clear_time: r.stage_clear_time,
        })
        .collect();

    let total = evil_castle_total_info::get_evil_castle_total_info(pool, uid)
        .await
        .ok()
        .and_then(|v| v.into_iter().next());

    let response = EvilCastleInfoResponse {
        evil_castle_info: evil_castle_info_list,
        evil_castle_total_info: total.map(|t| EvilCastleTotalDbInfo {
            rank: None,
            point: t.point,
            is_rewarded: t.is_rewarded,
        }),
        season: Some(CURRENT_SEASON),
        regular_season: Some(CURRENT_SEASON),
    };

    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::EvilCastleInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
