use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameRhythmInfoRequest, MiniGameRhythmInfoResponse, MiniGameRhythmPlayDbInfo, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_rhythm_play_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account best-record read.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameRhythmInfoRequest) -> GameResponse {
    info!("Handling MiniGameRhythmInfoRequest: {:?}", req);

    let play_info = db::get_mini_game_rhythm_play_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| MiniGameRhythmPlayDbInfo { id: r.id, mode_type: r.mode_type, best_record_value: r.best_record_value, best_grade_type: r.best_grade_type, best_combo_type: r.best_combo_type })
        .collect();

    let response = MiniGameRhythmInfoResponse { play_info, reward_info_bundle: None };
    
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
    
    let (route, code) = PacketCodeType::MiniGameRhythmInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}