use bd2::prost::Message;
use bd2::proto::proto_net::{TotalWarRewardStateRequest, TotalWarRewardStateResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::total::total_war_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, parse_scores, total_score};

/// Judgment call: the proto field is literally "is_obtainable_daily_reward" but nothing in
/// this system (real TotalWarRewardTable) distinguishes a "daily" reward from the ordinary
/// score-tier rewards TotalWarReward already claims — interpreted as "is there at least one
/// unclaimed score tier available right now", which is the closest real, checkable meaning.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: TotalWarRewardStateRequest,
) -> GameResponse {
    info!("Handling TotalWarRewardStateRequest: {:?}", req);

    let row = db::get_or_create(pool, uid).await.ok();
    let obtainable = match &row {
        Some(row) => {
            let score = total_score(&parse_scores(row));
            let claimed: Vec<i32> = row
                .claimed_reward_ids
                .as_deref()
                .and_then(|s| serde_json::from_str(s).ok())
                .unwrap_or_default();
            data::exceldb::get()
                .totalwarrewardtable
                .iter()
                .any(|t| (score as f32) >= t.score && !claimed.contains(&t.id))
        }
        None => false,
    };

    let response = TotalWarRewardStateResponse {
        is_obtainable_daily_reward: Some(obtainable),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::TotalWarRewardState.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
