use bd2::prost::Message;
use bd2::proto::proto_net::{
    MiniGameHopscotchGameEndRequest, MiniGameHopscotchGameEndResponse, MiniGameHopscotchStageDbInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::{
    mini_game_hopscotch_gallery_info, mini_game_hopscotch_play_state, mini_game_hopscotch_record_info,
};
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, HOPSCOTCH_CLEAR_REWARD_IDS};

/// No `IBStageTable`-equivalent clear-condition data exists for Hopscotch, so any submitted
/// result is treated as a completed run (documented placeholder) — captured_area/clear_time are
/// real, reported values, stored and ranked genuinely; only the "did this count as a clear"
/// rule and the reward id are placeholders.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameHopscotchGameEndRequest) -> GameResponse {
    info!("Handling MiniGameHopscotchGameEndRequest: {:?}", req);

    let play_state = mini_game_hopscotch_play_state::get(pool, uid).await.ok().flatten();
    let (event_schedule_id, stage_id) = match play_state {
        Some(s) => (s.event_schedule_id, s.stage_id),
        None => (0, 0),
    };
    let captured_area = req.captured_area.unwrap_or_default();
    let clear_time = req.clear_time.unwrap_or_default();

    let _ = mini_game_hopscotch_record_info::submit_result(
        pool,
        uid,
        event_schedule_id,
        stage_id,
        captured_area,
        clear_time,
    )
    .await;

    let newly_unlocked =
        mini_game_hopscotch_gallery_info::unlock(pool, uid, event_schedule_id, stage_id)
            .await
            .unwrap_or(false);

    let response = MiniGameHopscotchGameEndResponse {
        acquired_gallery_id: if newly_unlocked { vec![stage_id] } else { vec![] },
        stage_info: Some(MiniGameHopscotchStageDbInfo {
            stage_id: Some(stage_id),
            is_clear: Some(true),
        }),
        reward_id: HOPSCOTCH_CLEAR_REWARD_IDS.to_vec(),
        reward_bundle_info: None,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameHopscotchGameEnd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
