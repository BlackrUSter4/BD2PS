use bd2::prost::Message;
use bd2::proto::proto_net::{CharPartnerRewardRequest, CharPartnerRewardResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::char::char_partner_info;
use sqlx::SqlitePool;
use tracing::info;

/// `inven_index` is read as the `CharPartnerInfo` row's own rowid (`Index`)
/// — the message shape doesn't otherwise distinguish "which pairing", and no
/// other field maps cleanly to it. `Reward` is used as a claimed-steps
/// bitmask (bit N = reward_step N claimed) to prevent double-granting; the
/// actual reward CONTENTS are a documented placeholder (no per-step reward
/// table exists anywhere).
pub async fn handle(pool: &SqlitePool, uid: i64, req: CharPartnerRewardRequest) -> GameResponse {
    info!("Handling CharPartnerRewardRequest: {:?}", req);

    let (Some(index), Some(reward_step)) = (req.inven_index, req.reward_step) else {
        return GameResponse::error(1);
    };
    if !(0..31).contains(&reward_step) {
        return GameResponse::error(1);
    }

    let Ok(row) = char_partner_info::get_by_index(pool, uid, index).await else {
        return GameResponse::error(1);
    };

    let bit = 1i32 << reward_step;
    let current = row.reward.unwrap_or(0);
    if current & bit != 0 {
        // Already claimed — no double grant.
        let response = CharPartnerRewardResponse::default();
        let resp_bytes = response.encode_to_vec();
        let (route, code) = PacketCodeType::CharPartnerReward.info();
        return GameResponse::success(route, &resp_bytes, code);
    }

    let (Some(main), Some(sub)) = (row.main_unique_id, row.sub_unique_id) else {
        return GameResponse::error(1);
    };
    let _ = char_partner_info::set_reward_bits(pool, uid, main, sub, current | bit).await;

    let response = CharPartnerRewardResponse::default();
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::CharPartnerReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
