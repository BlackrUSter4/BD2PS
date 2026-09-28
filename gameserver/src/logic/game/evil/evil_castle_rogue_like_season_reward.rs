use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeSeasonRewardRequest, EvilCastleRogueLikeSeasonRewardResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::evil::{evil_castle_rogue_like_info, evil_castle_rogue_like_score_info};
use sqlx::SqlitePool;
use tracing::info;

/// Claims the season-end reward tier matching the account's best score,
/// via RLRewardTable's real score thresholds/reward arrays.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeSeasonRewardRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeSeasonRewardRequest: {:?}", req);
    let _ = req;

    let score = evil_castle_rogue_like_score_info::get_one(pool, uid).await.ok().flatten().and_then(|s| s.total_score).unwrap_or(0) as f32;

    let table = &exceldb::get().rlrewardtable;
    let mut best_rank = 0i32;
    let mut best_row = None;
    for (i, row) in table.all().iter().enumerate() {
        if score >= row.score {
            best_rank = i as i32 + 1;
            best_row = Some(row);
        }
    }

    let reward_info_bundle = if let Some(row) = best_row {
        Some(super::grant_rewards(pool, uid, &row.reward_id, &row.reward_type, &row.reward_count).await)
    } else {
        None
    };

    if let Some(mut run) = evil_castle_rogue_like_info::get_one(pool, uid).await.ok().flatten() {
        run.season_reward = Some(1);
        let _ = evil_castle_rogue_like_info::upsert(pool, &run).await;
    }

    let response = EvilCastleRogueLikeSeasonRewardResponse { rank: Some(best_rank), reward_info_bundle };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeSeasonReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
