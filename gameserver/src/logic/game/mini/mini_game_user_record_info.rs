use bd2::prost::Message;
use bd2::proto::proto_net::{
    MiniGameHopscotchUserRecordDbInfo, MiniGameUserRecordInfoRequest, MiniGameUserRecordInfoResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::{
    mini_game_action_multi_rank_info, mini_game_action_single_rank_info,
    mini_game_hopscotch_record_info, mini_game_rhythm_rank_info, mini_game_sichuan_rank_info,
    mini_game_survival_rank_info,
};
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Aggregate personal-record view across every minigame type. Hopscotch is real end-to-end;
/// the rest reflect real (currently-empty) tables — see mini_game_action_user_record_info.rs
/// for why they're empty right now.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameUserRecordInfoRequest) -> GameResponse {
    info!("Handling MiniGameUserRecordInfoRequest: {:?}", req);

    let survival_record = mini_game_survival_rank_info::get_mini_game_survival_rank_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .max_by_key(|r| r.point)
        .map(|r| bd2::proto::proto_net::MiniGameSurvivalUserRecordDbInfo {
            point: r.point,
            top_percent: None,
        });

    let sichuan_record = mini_game_sichuan_rank_info::get_mini_game_sichuan_rank_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .max_by(|a, b| {
            a.score
                .unwrap_or(0.0)
                .partial_cmp(&b.score.unwrap_or(0.0))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|r| bd2::proto::proto_net::MiniGameSichuanUserRecordDbInfo {
            score: r.score,
            top_percent: None,
        });

    let action_single_record = mini_game_action_single_rank_info::get_mini_game_action_single_rank_info(
        pool, uid,
    )
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|r| bd2::proto::proto_net::MiniGameActionUserRecordDbInfo {
        action_monster: None,
        score: r.score,
        top_percent: None,
        user_info: vec![],
    })
    .collect();

    let action_multi_record = mini_game_action_multi_rank_info::get_mini_game_action_multi_rank_info(
        pool, uid,
    )
    .await
    .unwrap_or_default()
    .into_iter()
    .map(|r| bd2::proto::proto_net::MiniGameActionUserRecordDbInfo {
        action_monster: None,
        score: r.score,
        top_percent: None,
        user_info: vec![],
    })
    .collect();

    let rhythm_record = mini_game_rhythm_rank_info::get_mini_game_rhythm_rank_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| bd2::proto::proto_net::MiniGameRhythmUserRecordDbInfo {
            point: r.point,
            rhythm_game_id: None,
            top_percent: None,
        })
        .collect();

    let hopscotch_rows = mini_game_hopscotch_record_info::list_all_for_uid(pool, uid)
        .await
        .unwrap_or_default();
    let mut hopscotch_record: Vec<MiniGameHopscotchUserRecordDbInfo> = Vec::new();
    for r in hopscotch_rows {
        let top_percent = mini_game_hopscotch_record_info::percentile(
            pool,
            r.event_schedule_id,
            r.stage_id,
            r.captured_area,
            r.clear_time,
        )
        .await
        .ok()
        .flatten();
        hopscotch_record.push(MiniGameHopscotchUserRecordDbInfo {
            stage_id: Some(r.stage_id),
            captured_area: Some(r.captured_area),
            clear_time: Some(r.clear_time),
            top_percent,
        });
    }

    let response = MiniGameUserRecordInfoResponse {
        survival_record,
        sichuan_record,
        action_single_record,
        action_multi_record,
        rhythm_record,
        hopscotch_record,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameUserRecordInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
