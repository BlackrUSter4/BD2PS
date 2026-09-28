use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameRhythmRankDbInfo, MiniGameRhythmRankingRequest, MiniGameRhythmRankingResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_rhythm_rank_info as rank_db;
use sqlx::SqlitePool;
use tracing::info;

const RANKING_LIMIT: i64 = 100;

/// Real cross-account leaderboard.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: MiniGameRhythmRankingRequest,
) -> GameResponse {
    info!("Handling MiniGameRhythmRankingRequest: {:?}", req);

    let top = rank_db::get_top(pool, RANKING_LIMIT).await.unwrap_or_default();
    let rank_info = top
        .iter()
        .enumerate()
        .map(|(i, r)| MiniGameRhythmRankDbInfo { rank: Some(i as i32 + 1), owner_index: r.owner_index, user_id: r.user_id.clone(), point: r.point })
        .collect();

    let user_rank_info = rank_db::get_own(pool, uid).await.ok().flatten().map(|r| {
        let rank = top.iter().position(|t| t.uid == r.uid).map(|p| p as i32 + 1);
        MiniGameRhythmRankDbInfo { rank, owner_index: r.owner_index, user_id: r.user_id, point: r.point }
    });

    let response = MiniGameRhythmRankingResponse { rank_info, user_rank_info };

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

    let (route, code) = PacketCodeType::MiniGameRhythmRanking.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
