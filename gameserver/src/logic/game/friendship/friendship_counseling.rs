use bd2::prost::Message;
use bd2::proto::proto_net::{FriendshipCounselingRequest, FriendshipCounselingResponse, FriendshipDbInfo, ItemDbInfo, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::friendship::{friendship_counseling_daily, friendship_counseling_session, friendship_info};
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

use super::{
    COUNSELING_COMPLETE_REWARD_COUNT, COUNSELING_COMPLETE_REWARD_ITEM_ID, COUNSELING_COMPLETE_REWARD_ITEM_TYPE,
    COUNSELING_CORRECT_EXP,
};

/// No counseling-question/answer master data was captured (there is no table anywhere linking a
/// `(session_id, select_index)` pair to a correct answer), so every answer is treated as correct
/// — same "don't fabricate a failure state nothing confirms" judgment call used elsewhere in this
/// project (e.g. Ib's always-clearable dungeon stage). `is_quick` doesn't change the exp amount;
/// no data distinguishes a quick-mode reward from a normal one.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FriendshipCounselingRequest) -> GameResponse {
    info!("Handling FriendshipCounselingRequest: {:?}", req);

    let costume_id = req.costume_id.unwrap_or_default();
    let session_id = req.session_id.unwrap_or_default();
    let is_correct = true;

    let allowed = friendship_counseling_daily::check_and_would_allow(pool, uid, costume_id).await;
    let gained_exp = if allowed { COUNSELING_CORRECT_EXP } else { 0 };

    let mut reward_info_bundle = RewardDbInfoBundle::default();

    if allowed {
        friendship_counseling_session::mark_done(pool, uid, costume_id, session_id).await;
        let reached_cap_unrewarded = friendship_counseling_daily::record_session(pool, uid, costume_id).await;
        if reached_cap_unrewarded {
            let _ = item_info::grant(
                pool,
                uid,
                COUNSELING_COMPLETE_REWARD_ITEM_ID,
                COUNSELING_COMPLETE_REWARD_ITEM_TYPE,
                COUNSELING_COMPLETE_REWARD_COUNT,
            )
            .await;
            friendship_counseling_daily::mark_completion_reward_granted(pool, uid).await;
            reward_info_bundle = RewardDbInfoBundle {
                item_info: vec![ItemDbInfo {
                    id: Some(COUNSELING_COMPLETE_REWARD_ITEM_ID),
                    r#type: Some(COUNSELING_COMPLETE_REWARD_ITEM_TYPE),
                    count: Some(COUNSELING_COMPLETE_REWARD_COUNT),
                    ..Default::default()
                }],
                ..Default::default()
            };
        }
    }

    let info_row = friendship_info::add_exp(pool, uid, costume_id, gained_exp, true).await;
    let friendship_info_out = FriendshipDbInfo {
        friendship_costume_id: Some(info_row.costume_id),
        level: Some(info_row.level),
        exp: Some(info_row.exp),
        last_counseling_date: info_row.last_counseling_date,
    };

    let response = FriendshipCounselingResponse {
        reward_info_bundle: Some(reward_info_bundle),
        friendship_info: Some(friendship_info_out),
        is_correct: Some(is_correct),
        gained_exp: Some(gained_exp),
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::FriendshipCounseling.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
