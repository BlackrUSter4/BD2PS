use bd2::prost::Message;
use bd2::proto::proto_net::{
    MiniGameHopscotchRankingRankDbInfo, MiniGameRankingRequest, MiniGameRankingResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_hopscotch_record_info;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, HOPSCOTCH_RANKING_LIMIT};

/// Global (un-scoped) ranking view. Hopscotch is real cross-account data; the other minigame
/// types' ranking lists are genuinely empty (their gameplay handlers are unfilled TODO stubs
/// that never write ranking rows — see mini_game_action_user_record_info.rs).
pub async fn handle(pool: &SqlitePool, _uid: i64, req: MiniGameRankingRequest) -> GameResponse {
    info!("Handling MiniGameRankingRequest: {:?}", req);

    let rows = mini_game_hopscotch_record_info::top_overall(pool, HOPSCOTCH_RANKING_LIMIT)
        .await
        .unwrap_or_default();

    let mut hopscotch_ranking_list = Vec::with_capacity(rows.len());
    for r in &rows {
        hopscotch_ranking_list.push(MiniGameHopscotchRankingRankDbInfo {
            stage_id: Some(r.stage_id),
            user_id: Some(crate::logic::game::display_name(pool, r.uid).await),
            captured_area: Some(r.captured_area),
            clear_time: Some(r.clear_time),
        });
    }

    let response = MiniGameRankingResponse {
        survival_ranking: None,
        sichuan_ranking: None,
        action_single_ranking_list: vec![],
        action_multi_ranking_list: vec![],
        rhythm_ranking_list: vec![],
        hopscotch_ranking_list,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameRanking.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
