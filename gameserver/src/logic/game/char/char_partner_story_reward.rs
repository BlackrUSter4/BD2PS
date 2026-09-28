use bd2::prost::Message;
use bd2::proto::proto_net::{
    CharPartnerStoryRewardRequest, CharPartnerStoryRewardResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::char::{char_info, char_partner_info};
use sqlx::SqlitePool;
use tracing::info;

/// `inven_index`/`partner_char_index` are both character InvenIndex values
/// (main + partner); resolved to their template ids to find-or-create the
/// `CharPartnerInfo` pairing row. Bit 0 of `Reward` is used as the
/// "story unlocked" flag (distinct from CharPartnerReward's per-step bits,
/// which start at bit 1) — a documented judgment call since no table
/// distinguishes the two claim types. Reward CONTENTS are placeholder-empty.
pub async fn handle(pool: &SqlitePool, uid: i64, req: CharPartnerStoryRewardRequest) -> GameResponse {
    info!("Handling CharPartnerStoryRewardRequest: {:?}", req);

    let (Some(main_inven), Some(sub_inven)) = (req.inven_index, req.partner_char_index) else {
        return GameResponse::error(1);
    };
    let Ok(Some(main_char)) = char_info::get_by_inven_index(pool, uid, main_inven).await else {
        return GameResponse::error(1);
    };
    let Ok(Some(sub_char)) = char_info::get_by_inven_index(pool, uid, sub_inven).await else {
        return GameResponse::error(1);
    };
    let (Some(main_id), Some(sub_id)) = (main_char.id, sub_char.id) else {
        return GameResponse::error(1);
    };

    let existing = char_partner_info::get_by_pair(pool, uid, main_id, sub_id)
        .await
        .ok()
        .flatten();

    let already_claimed = existing
        .as_ref()
        .map(|r| r.reward.unwrap_or(0) & 1 != 0)
        .unwrap_or(false);

    if let Some(row) = existing {
        if !already_claimed {
            let new_reward = row.reward.unwrap_or(0) | 1;
            let _ = char_partner_info::set_reward_bits(pool, uid, main_id, sub_id, new_reward).await;
        }
    } else {
        let row = database::models::game::char::char_partner_info::CharPartnerInfo {
            index: 0,
            uid,
            main_unique_id: Some(main_id),
            sub_unique_id: Some(sub_id),
            point: Some(0),
            reward: Some(1),
        };
        let _ = char_partner_info::insert(pool, &row).await;
    }

    let response = CharPartnerStoryRewardResponse::default();
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::CharPartnerStoryReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
