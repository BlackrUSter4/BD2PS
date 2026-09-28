use bd2::prost::Message;
use bd2::proto::proto_net::{CharSpecialScoutResetRequest, CharSpecialScoutResetResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::char::char_scout_info;
use rand::prelude::IndexedRandom;
use sqlx::SqlitePool;
use tracing::info;

/// Real reset-limit enforcement and lineup regeneration using
/// `SpecialScoutInfoTable`'s real `resetLimitCount`/`autoResetMinute`. The
/// request carries no items to consume, and no generic currency-deduction
/// mechanism exists anywhere else in this codebase to spend the table's
/// `resetCostType`/`resetCostCount` against — so the cost itself is NOT
/// enforced (documented placeholder); everything else is real.
pub async fn handle(pool: &SqlitePool, uid: i64, req: CharSpecialScoutResetRequest) -> GameResponse {
    info!("Handling CharSpecialScoutResetRequest: {:?}", req);

    let game_data = exceldb::get();
    let config = game_data.specialscoutinfotable.all().first();
    let appear_total = config.map(|c| c.appear_total_count).unwrap_or(2).max(1) as usize;
    let reset_limit = config.map(|c| c.reset_limit_count).unwrap_or(99);
    let auto_reset_minute = config.map(|c| c.auto_reset_minute).unwrap_or(120) as i64;

    let rows = char_scout_info::get_char_scout_info(pool, uid)
        .await
        .unwrap_or_default();
    let use_reset_count = rows.first().and_then(|r| r.use_reset_count).unwrap_or(0);
    if use_reset_count >= reset_limit {
        return GameResponse::error(1);
    }

    let mut rng = rand::thread_rng();
    let ids: Vec<i32> = game_data
        .chartable
        .all()
        .choose_multiple(&mut rng, appear_total)
        .map(|c| c.id)
        .collect();

    let now = chrono::Utc::now().timestamp_millis();
    let next_reset = now + auto_reset_minute * 60_000;
    let _ = char_scout_info::replace_lineup(pool, uid, &ids, use_reset_count + 1, next_reset).await;

    let response = CharSpecialScoutResetResponse {
        appear_char_id: ids,
        next_auto_reset_time: Some(next_reset),
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
    let (route, code) = PacketCodeType::CharSpecialScoutReset.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
