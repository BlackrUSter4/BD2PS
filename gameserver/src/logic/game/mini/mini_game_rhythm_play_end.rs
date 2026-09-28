use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameRhythmPlayDbInfo, MiniGameRhythmPlayEndRequest, MiniGameRhythmPlayEndResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{mini::{mini_game_rhythm_play_info as play_db, mini_game_rhythm_rank_info as rank_db}, user::user_info as user_db};
use sqlx::SqlitePool;
use tracing::info;

/// Real best-record persistence (judgment call: this request has no integer chart id, only a
/// `game_id` string, so it's parsed as the play-info key; `mode_type` isn't in this request at
/// all, so it defaults to 0). Real cross-account rank point update.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameRhythmPlayEndRequest) -> GameResponse {
    info!("Handling MiniGameRhythmPlayEndRequest: {:?}", req);

    let id: i32 = req.game_id.as_deref().and_then(|s| s.parse().ok()).unwrap_or(0);
    let score = req.score.unwrap_or(0);
    let row = play_db::upsert_best(pool, uid, id, 0, score, req.grade_type.unwrap_or(0), req.combo_type.unwrap_or(0)).await.ok();

    let user = user_db::get_user_info(pool, uid).await.unwrap_or_default().into_iter().next();
    let owner_index = user.as_ref().and_then(|u| u.owner_index).unwrap_or(0);
    let user_id = user.as_ref().and_then(|u| u.user_id.clone()).unwrap_or_default();
    let _ = rank_db::upsert_best(pool, uid, owner_index, &user_id, score as i64).await;

    let play_info = row.map(|r| MiniGameRhythmPlayDbInfo { id: r.id, mode_type: r.mode_type, best_record_value: r.best_record_value, best_grade_type: r.best_grade_type, best_combo_type: r.best_combo_type });

    let response = MiniGameRhythmPlayEndResponse { play_info };
    
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
    
    let (route, code) = PacketCodeType::MiniGameRhythmPlayEnd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}