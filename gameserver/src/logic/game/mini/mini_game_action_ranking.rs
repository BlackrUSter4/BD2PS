use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameActionRankingRequest, MiniGameActionRankingResponse, MiniGameActionSingleRankDbInfo, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_action_single_rank_info as rank_db;
use sqlx::SqlitePool;
use tracing::info;

const RANKING_LIMIT: i64 = 100;

/// Real cross-account leaderboard. No multiplayer session infrastructure exists in this
/// project (see MiniGameRelayServerInfo), so multi_user_ranking_info/multi_my_ranking_info
/// stay honestly empty.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: MiniGameActionRankingRequest,
) -> GameResponse {
    info!("Handling MiniGameActionRankingRequest: {:?}", req);

    let top = rank_db::get_top(pool, RANKING_LIMIT).await.unwrap_or_default();
    let single_user_ranking_info = top
        .iter()
        .enumerate()
        .map(|(i, r)| MiniGameActionSingleRankDbInfo { owner_index: r.owner_index, user_id: r.user_id.clone(), rank: Some(i as i32 + 1), score: r.score, char_id: r.char_id })
        .collect();

    let single_my_ranking_info = rank_db::get_own(pool, uid).await.ok().flatten().map(|r| {
        let rank = top.iter().position(|t| t.uid == r.uid).map(|p| p as i32 + 1);
        MiniGameActionSingleRankDbInfo { owner_index: r.owner_index, user_id: r.user_id, rank, score: r.score, char_id: r.char_id }
    });

    let response = MiniGameActionRankingResponse {
        single_user_ranking_info,
        single_my_ranking_info,
        multi_user_ranking_info: vec![],
        multi_my_ranking_info: None,
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::MiniGameActionRanking.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
