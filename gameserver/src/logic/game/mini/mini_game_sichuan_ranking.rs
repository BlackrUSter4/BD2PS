use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameSichuanRankDbInfo, MiniGameSichuanRankingRequest, MiniGameSichuanRankingResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_sichuan_rank_info as rank_db;
use sqlx::SqlitePool;
use tracing::info;

const RANKING_LIMIT: i64 = 100;

/// Real cross-account leaderboard. user_exp/portrait/guild fields aren't joined in this pass
/// (would need cross-referencing UserInfo/Guild for every ranked account) — left honestly
/// empty; rank/score/owner_index/user_id are real.
pub async fn handle(pool: &SqlitePool, _uid: i64, req: MiniGameSichuanRankingRequest) -> GameResponse {
    info!("Handling MiniGameSichuanRankingRequest: {:?}", req);

    let top = rank_db::get_top(pool, RANKING_LIMIT).await.unwrap_or_default();
    let user_ranking_info = top
        .iter()
        .enumerate()
        .map(|(i, r)| MiniGameSichuanRankDbInfo {
            owner_index: r.owner_index,
            user_id: r.user_id.clone(),
            user_exp: None,
            portrait_costume_id: None,
            portrait_costume_design_id: None,
            guild_base_info: None,
            rank: Some(i as i32 + 1),
            score: r.score,
            title_id: r.title_id,
        })
        .collect();

    let response = MiniGameSichuanRankingResponse { user_ranking_info };
    
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
    
    let (route, code) = PacketCodeType::MiniGameSichuanRanking.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}