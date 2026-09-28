use bd2::prost::Message;
use bd2::proto::proto_net::{
    MiniGameHopscotchUserRecordDbInfo, MiniGameHopscotchUserRecordInfoRequest,
    MiniGameHopscotchUserRecordInfoResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_hopscotch_record_info;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: MiniGameHopscotchUserRecordInfoRequest,
) -> GameResponse {
    info!("Handling MiniGameHopscotchUserRecordInfoRequest: {:?}", req);
    let event_schedule_id = req.event_schedule_id.unwrap_or_default();
    let stage_id = req.stage_id.unwrap_or_default();

    let record = mini_game_hopscotch_record_info::get(pool, uid, event_schedule_id, stage_id)
        .await
        .ok()
        .flatten();

    let record_info = match record {
        Some(r) => {
            let top_percent = mini_game_hopscotch_record_info::percentile(
                pool,
                event_schedule_id,
                stage_id,
                r.captured_area,
                r.clear_time,
            )
            .await
            .ok()
            .flatten();
            Some(MiniGameHopscotchUserRecordDbInfo {
                stage_id: Some(r.stage_id),
                captured_area: Some(r.captured_area),
                clear_time: Some(r.clear_time),
                top_percent,
            })
        }
        None => None,
    };

    let response = MiniGameHopscotchUserRecordInfoResponse { record_info };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameHopscotchUserRecordInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
