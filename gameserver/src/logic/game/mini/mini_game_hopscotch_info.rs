use bd2::prost::Message;
use bd2::proto::proto_net::{
    MiniGameHopscotchInfoRequest, MiniGameHopscotchInfoResponse, MiniGameHopscotchStageDbInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::{mini_game_hopscotch_gallery_info, mini_game_hopscotch_record_info};
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, HOPSCOTCH_CLEAR_REWARD_IDS, HOPSCOTCH_STAGE_IDS};

pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameHopscotchInfoRequest) -> GameResponse {
    info!("Handling MiniGameHopscotchInfoRequest: {:?}", req);
    let event_schedule_id = req.event_schedule_id.unwrap_or_default();

    let gallery = mini_game_hopscotch_gallery_info::list_for_event(pool, uid, event_schedule_id)
        .await
        .unwrap_or_default();
    let records = mini_game_hopscotch_record_info::list_for_event(pool, uid, event_schedule_id)
        .await
        .unwrap_or_default();

    let stage_info = HOPSCOTCH_STAGE_IDS
        .iter()
        .map(|&stage_id| {
            let is_clear = records.iter().any(|r| r.stage_id == stage_id && r.is_clear);
            MiniGameHopscotchStageDbInfo {
                stage_id: Some(stage_id),
                is_clear: Some(is_clear),
            }
        })
        .collect();

    let response = MiniGameHopscotchInfoResponse {
        gallery_id: gallery.iter().map(|g| g.gallery_id).collect(),
        stage_info,
        reward_id: HOPSCOTCH_CLEAR_REWARD_IDS.to_vec(),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameHopscotchInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
