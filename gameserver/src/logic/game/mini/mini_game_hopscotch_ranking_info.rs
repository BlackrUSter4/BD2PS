use bd2::prost::Message;
use bd2::proto::proto_net::{
    MiniGameHopscotchRankingDbInfo, MiniGameHopscotchRankingInfoRequest,
    MiniGameHopscotchRankingInfoResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_hopscotch_record_info;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, HOPSCOTCH_RANKING_LIMIT};

pub async fn handle(
    pool: &SqlitePool,
    _uid: i64,
    req: MiniGameHopscotchRankingInfoRequest,
) -> GameResponse {
    info!("Handling MiniGameHopscotchRankingInfoRequest: {:?}", req);

    let rows = mini_game_hopscotch_record_info::ranking_for_stage(
        pool,
        req.event_schedule_id.unwrap_or_default(),
        req.stage_id.unwrap_or_default(),
        HOPSCOTCH_RANKING_LIMIT,
    )
    .await
    .unwrap_or_default();

    let mut ranking_info = Vec::with_capacity(rows.len());
    for (idx, r) in rows.iter().enumerate() {
        ranking_info.push(MiniGameHopscotchRankingDbInfo {
            rank: Some(idx as i32 + 1),
            owner_index: Some(r.uid),
            user_id: Some(crate::logic::game::display_name(pool, r.uid).await),
            captured_area: Some(r.captured_area),
            clear_time: Some(r.clear_time),
        });
    }

    let response = MiniGameHopscotchRankingInfoResponse { ranking_info };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameHopscotchRankingInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
