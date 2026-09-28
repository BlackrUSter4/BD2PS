use bd2::prost::Message;
use bd2::proto::proto_net::{
    MiniGameSurvivalUserRecordDbInfo, MiniGameSurvivalUserRecordInfoRequest,
    MiniGameSurvivalUserRecordInfoResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_survival_rank_info;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// See mini_game_action_user_record_info.rs: real query, currently always-empty since the
/// survival gameplay handlers that would populate this are unfilled TODO stubs.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: MiniGameSurvivalUserRecordInfoRequest,
) -> GameResponse {
    info!("Handling MiniGameSurvivalUserRecordInfoRequest: {:?}", req);

    let record_info = mini_game_survival_rank_info::get_mini_game_survival_rank_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .max_by_key(|r| r.point)
        .map(|r| MiniGameSurvivalUserRecordDbInfo {
            point: r.point,
            top_percent: None,
        });

    let response = MiniGameSurvivalUserRecordInfoResponse { record_info };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameSurvivalUserRecordInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
