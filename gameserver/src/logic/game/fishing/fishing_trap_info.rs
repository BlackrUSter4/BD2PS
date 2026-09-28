use super::{now_ms, level_for_exp, CATCH_EXP};
use bd2::prost::Message;
use bd2::proto::proto_net::{FishingTrapDbInfo, FishingTrapInfoRequest, FishingTrapInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

const TRAP_CATCH_INTERVAL_MS: i64 = 60 * 60 * 1000; // one passive catch per hour — placeholder

/// Previews (and lazily tops up) passive "trap" catches accrued since the last claim. The
/// interval-per-catch is a flagged placeholder, but the *cap* on total accrual
/// (`fishTrapMaxTime`, 86400s = 24h) comes from real captured `FishingDefaultTable` data, so
/// the ceiling is genuine even though the fill rate is a guess. `fishTrapFishPoolId` (999 in
/// the captured row) is a pool id, not a real fish id — reused directly as a placeholder
/// fish_id since no `FishingFishPoolTable` exists to resolve it properly.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingTrapInfoRequest) -> GameResponse {
    info!("Handling FishingTrapInfoRequest: {:?}", req);

    let now = now_ms();
    let user = database::db::fishing::fishing_user_info::get_or_create(pool, uid)
        .await
        .unwrap_or_default();
    let last_time = user.trap_reward_receipt_time.unwrap_or(now);
    if user.trap_reward_receipt_time.is_none() {
        let _ = database::db::fishing::fishing_user_info::set_trap_reward_receipt_time(pool, uid, now).await;
    }

    let default_row = data::exceldb::get().fishingdefaulttable.all().first().cloned();
    let max_time_ms = default_row.as_ref().map(|d| d.fish_trap_max_time as i64 * 1000).unwrap_or(86_400_000);
    let fish_id = default_row.as_ref().map(|d| d.fish_trap_fish_pool_id).unwrap_or(1);

    let elapsed_ms = (now - last_time).clamp(0, max_time_ms);
    let target_count = (elapsed_ms / TRAP_CATCH_INTERVAL_MS) as i64;

    let mut pending = database::db::fishing::fishing_trap_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();
    while (pending.len() as i64) < target_count {
        use rand::Rng;
        let size = rand::thread_rng().gen_range(100..=999);
        let _ = database::db::fishing::fishing_trap_info::add(pool, uid, fish_id, size).await;
        pending.push(database::models::game::fishing::fishing_trap_info::FishingTrapInfo {
            index: 0,
            uid,
            fish_id,
            size,
        });
    }

    let total_add_exp = pending.len() as i32 * CATCH_EXP;
    let after_exp = user.exp + total_add_exp;
    let after_level = level_for_exp(after_exp);

    let response = FishingTrapInfoResponse {
        trap_reward_receipt_time: Some(last_time),
        fish_trap_info: pending
            .iter()
            .map(|r| FishingTrapDbInfo { fish_id: Some(r.fish_id), size: Some(r.size) })
            .collect(),
        total_add_exp: Some(total_add_exp),
        after_level: Some(after_level),
        after_exp: Some(after_exp),
    };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::FishingTrapInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
