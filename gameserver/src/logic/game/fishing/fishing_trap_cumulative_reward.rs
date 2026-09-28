use super::{now_ms, level_for_exp};
use bd2::prost::Message;
use bd2::proto::proto_net::{FishingTrapCumulativeRewardRequest, FishingTrapCumulativeRewardResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

const EXP_PER_TRAP_CATCH: i32 = 10; // matches CATCH_EXP; kept as its own const for clarity here

/// Claims everything FishingTrapInfo has accrued. When `is_auto_sell_enabled` is set, the
/// real game presumably auto-sells each catch for currency — no price table was captured, so
/// this deliberately grants nothing for the auto-sold catches rather than inventing a price
/// (flagged); when disabled, each catch is granted into the real fish inventory + collection
/// log exactly like a normal catch.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingTrapCumulativeRewardRequest) -> GameResponse {
    info!("Handling FishingTrapCumulativeRewardRequest: {:?}", req);

    let now = now_ms();
    let auto_sell = req.is_auto_sell_enabled.unwrap_or(false);
    let pending = database::db::fishing::fishing_trap_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();

    if !auto_sell {
        for row in &pending {
            let _ = database::db::fishing::fishing_fish_info::insert(pool, uid, row.fish_id, row.size, now).await;
            let _ = database::db::fishing::fishing_collection_info::record_catch(pool, uid, row.fish_id, row.size, now).await;
        }
    }
    let _ = database::db::fishing::fishing_trap_info::clear_all(pool, uid).await;
    let _ = database::db::fishing::fishing_user_info::set_trap_reward_receipt_time(pool, uid, now).await;

    let total_add_exp = pending.len() as i32 * EXP_PER_TRAP_CATCH;
    let user = database::db::fishing::fishing_user_info::get_or_create(pool, uid)
        .await
        .unwrap_or_default();
    let new_exp = user.exp + total_add_exp;
    let new_level = level_for_exp(new_exp);
    let _ = database::db::fishing::fishing_user_info::add_exp(pool, uid, total_add_exp, new_level).await;

    let response = FishingTrapCumulativeRewardResponse {
        trap_reward_receipt_time: Some(now),
        reward_info: None,
        level: Some(new_level),
        exp: Some(new_exp),
        total_add_exp: Some(total_add_exp),
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
    let (route, code) = PacketCodeType::FishingTrapCumulativeReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
