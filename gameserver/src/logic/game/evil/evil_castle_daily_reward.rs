use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleDailyRewardRequest, EvilCastleDailyRewardResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::evil::evil_castle_daily_reward_info;
use sqlx::SqlitePool;
use tracing::info;

/// EvilCastleDailyRewardTable's 9 rows are a login-streak ladder (real
/// reward per day) — this account's own claim count picks which row,
/// cycling once the streak runs past the table's length.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleDailyRewardRequest) -> GameResponse {
    info!("Handling EvilCastleDailyRewardRequest: {:?}", req);

    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let existing = evil_castle_daily_reward_info::get(pool, uid).await.ok().flatten();
    let already_claimed = existing
        .as_ref()
        .and_then(|r| r.last_claim_date.clone())
        .map(|d| d == today)
        .unwrap_or(false);

    let mut reward_info_bundle = None;
    if !already_claimed {
        let claim_count = existing.map(|r| r.claim_count).unwrap_or(0) + 1;
        let table = &exceldb::get().evilcastledailyrewardtable;
        if !table.all().is_empty() {
            let row_id = ((claim_count - 1).rem_euclid(table.all().len() as i32)) + 1;
            if let Some(r) = table.get(row_id) {
                let bundle = super::grant_rewards(pool, uid, &[r.reward_id], &[r.reward_type], &[r.reward_count]).await;
                reward_info_bundle = Some(bundle);
            }
        }
        let _ = evil_castle_daily_reward_info::set_claimed(pool, uid, &today, claim_count).await;
    }

    let response = EvilCastleDailyRewardResponse { reward_info_bundle };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::EvilCastleDailyReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
