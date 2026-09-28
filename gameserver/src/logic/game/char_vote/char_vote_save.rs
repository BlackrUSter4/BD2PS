use bd2::prost::Message;
use bd2::proto::proto_net::{
    CharVoteCountRewardDbInfo, CharVoteSaveRequest, CharVoteSaveResponse, CharVoteUserDbInfo, ItemDbInfo,
    RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::{
    default_notify, today_string, CURRENT_EVENT_ID, CURRENT_ROUND, PLACEHOLDER_REWARD_COUNT,
    PLACEHOLDER_REWARD_ITEM_ID, PLACEHOLDER_REWARD_ITEM_TYPE, REWARD_MILESTONES,
};

/// `vote_type == 0` is treated as a free daily ("normal") vote; anything else is a paid
/// ("additional") vote that consumes whatever items the client itself listed in `item_info` —
/// no CharVoteDefaultTable/cost table exists to say what a vote should actually cost, so the
/// client-supplied item list is trusted and consumed as-is rather than guessing a price to
/// validate it against.
pub async fn handle(pool: &SqlitePool, uid: i64, req: CharVoteSaveRequest) -> GameResponse {
    info!("Handling CharVoteSaveRequest: {:?}", req);

    let candidate_id = req.candidate_id.unwrap_or_default();
    let vote_count = req.vote_count.unwrap_or(1).max(1);
    let is_normal = req.vote_type.unwrap_or(0) == 0;

    let mut consume_item_info = Vec::new();
    for item in &req.item_info {
        if let (Some(id), Some(count)) = (item.id, item.count) {
            if database::db::item::item_info::consume(pool, uid, id, count).await.unwrap_or(false) {
                consume_item_info.push(item.clone());
            }
        }
    }

    let old_total = database::db::char_vote::char_vote_user_info::account_total_votes(pool, uid, CURRENT_EVENT_ID)
        .await
        .unwrap_or(0);

    let (normal_delta, additional_delta) = if is_normal { (vote_count, 0) } else { (0, vote_count) };
    let updated = database::db::char_vote::char_vote_user_info::add_vote(
        pool,
        uid,
        CURRENT_EVENT_ID,
        CURRENT_ROUND,
        candidate_id,
        normal_delta,
        additional_delta,
    )
    .await
    .unwrap_or_default();

    if is_normal {
        let today = today_string();
        let _ =
            database::db::char_vote::char_vote_daily_state::add_normal_vote_candidate(pool, uid, &today, candidate_id)
                .await;
    }

    let new_total = old_total + vote_count as i64;
    let mut new_reward_info = Vec::new();
    let mut granted_reward_items = Vec::new();
    for &milestone in REWARD_MILESTONES {
        if (milestone as i64) > old_total && (milestone as i64) <= new_total {
            if !database::db::char_vote::char_vote_reward_claim::has_claimed(pool, uid, CURRENT_EVENT_ID, milestone)
                .await
                .unwrap_or(true)
            {
                let _ = database::db::char_vote::char_vote_reward_claim::claim(pool, uid, CURRENT_EVENT_ID, milestone)
                    .await;
                let _ = database::db::item::item_info::grant(
                    pool,
                    uid,
                    PLACEHOLDER_REWARD_ITEM_ID,
                    PLACEHOLDER_REWARD_ITEM_TYPE,
                    PLACEHOLDER_REWARD_COUNT,
                )
                .await;
                new_reward_info.push(CharVoteCountRewardDbInfo {
                    event_id: Some(CURRENT_EVENT_ID),
                    reward_id: Some(milestone),
                });
                granted_reward_items.push(ItemDbInfo {
                    id: Some(PLACEHOLDER_REWARD_ITEM_ID),
                    r#type: Some(PLACEHOLDER_REWARD_ITEM_TYPE),
                    count: Some(PLACEHOLDER_REWARD_COUNT),
                    ..Default::default()
                });
            }
        }
    }

    let reward_info_bundle = if granted_reward_items.is_empty() {
        None
    } else {
        Some(RewardDbInfoBundle { item_info: granted_reward_items, ..Default::default() })
    };

    let response = CharVoteSaveResponse {
        reward_info_bundle,
        new_reward_info,
        consume_item_info,
        vote_info: Some(CharVoteUserDbInfo {
            round: Some(updated.round),
            candidate_id: Some(updated.candidate_id),
            total_count: Some(updated.total_count),
            normal_count: Some(updated.normal_count),
            additional_count: Some(updated.additional_count),
        }),
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CharVoteSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
