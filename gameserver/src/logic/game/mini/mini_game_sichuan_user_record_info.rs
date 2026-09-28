use bd2::prost::Message;
use bd2::proto::proto_net::{
    MiniGameSichuanUserRecordDbInfo, MiniGameSichuanUserRecordInfoRequest,
    MiniGameSichuanUserRecordInfoResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_sichuan_rank_info;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// See mini_game_action_user_record_info.rs: real query, currently always-empty since the
/// sichuan gameplay handlers that would populate this are unfilled TODO stubs.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: MiniGameSichuanUserRecordInfoRequest,
) -> GameResponse {
    info!("Handling MiniGameSichuanUserRecordInfoRequest: {:?}", req);

    let record_info = mini_game_sichuan_rank_info::get_mini_game_sichuan_rank_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .max_by(|a, b| {
            a.score
                .unwrap_or(0.0)
                .partial_cmp(&b.score.unwrap_or(0.0))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|r| MiniGameSichuanUserRecordDbInfo {
            score: r.score,
            top_percent: None,
        });

    let response = MiniGameSichuanUserRecordInfoResponse { record_info };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameSichuanUserRecordInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
