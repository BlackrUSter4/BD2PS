use bd2::prost::Message;
use bd2::proto::proto_net::{
    CharVoteCountRewardDbInfo, CharVoteInfoRequest, CharVoteInfoResponse, CharVoteRoundScheduleDbInfo,
    CharVoteUserDbInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::{
    candidate_ids, default_notify, next_midnight_utc_millis, today_string, CURRENT_EVENT_ID, CURRENT_ROUND,
};

pub async fn handle(pool: &SqlitePool, uid: i64, req: CharVoteInfoRequest) -> GameResponse {
    info!("Handling CharVoteInfoRequest: {:?}", req);

    let today = today_string();
    let before = database::db::char_vote::char_vote_daily_state::get(pool, uid)
        .await
        .ok()
        .flatten();
    let is_daily_reset = before.as_ref().map(|s| s.last_reset_date != today).unwrap_or(true);
    let daily_state = database::db::char_vote::char_vote_daily_state::reset_if_new_day(pool, uid, &today)
        .await
        .unwrap_or_default();

    let vote_rows =
        database::db::char_vote::char_vote_user_info::list_by_round(pool, uid, CURRENT_EVENT_ID, CURRENT_ROUND)
            .await
            .unwrap_or_default();
    let vote_info = vote_rows
        .into_iter()
        .map(|r| CharVoteUserDbInfo {
            round: Some(r.round),
            candidate_id: Some(r.candidate_id),
            total_count: Some(r.total_count),
            normal_count: Some(r.normal_count),
            additional_count: Some(r.additional_count),
        })
        .collect();

    let favorite_candidate_id = database::db::char_vote::char_vote_favorite::list(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|f| f.candidate_id)
        .collect();

    // reward_id == its milestone threshold, a documented placeholder mapping (see mod.rs) since
    // no CharVoteRewardTable exists to say what the real reward ids are.
    let mut reward_info = Vec::new();
    for &milestone in super::REWARD_MILESTONES {
        if database::db::char_vote::char_vote_reward_claim::has_claimed(pool, uid, CURRENT_EVENT_ID, milestone)
            .await
            .unwrap_or(false)
        {
            reward_info.push(CharVoteCountRewardDbInfo {
                event_id: Some(CURRENT_EVENT_ID),
                reward_id: Some(milestone),
            });
        }
    }

    let response = CharVoteInfoResponse {
        current_round: Some(CURRENT_ROUND),
        candidate_id: candidate_ids(),
        vote_info,
        reward_info,
        favorite_candidate_id,
        normal_vote_candidate_id: daily_state.candidate_ids(),
        next_daily_reset_time: Some(next_midnight_utc_millis()),
        is_daily_reset: Some(is_daily_reset),
        round_schedule_info: vec![CharVoteRoundScheduleDbInfo {
            round: Some(CURRENT_ROUND),
            start_time: Some(0),
            end_time: Some(next_midnight_utc_millis() + 30 * 24 * 60 * 60 * 1000),
        }],
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CharVoteInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
