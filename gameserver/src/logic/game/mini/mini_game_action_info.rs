use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameActionInfoRequest, MiniGameActionInfoResponse, MiniGameActionMyBestRecord, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_action_single_rank_info as rank_db;
use sqlx::SqlitePool;
use tracing::info;

/// Real read of the account's own best record.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameActionInfoRequest) -> GameResponse {
    info!("Handling MiniGameActionInfoRequest: {:?}", req);

    let own = rank_db::get_own(pool, uid).await.ok().flatten();
    let my_best_record = own
        .map(|r| vec![MiniGameActionMyBestRecord { monster_id: r.char_id, record: r.score.map(|s| s as i64) }])
        .unwrap_or_default();

    let response = MiniGameActionInfoResponse { event_schedule_id: None, my_best_record, clear_mission_id: vec![] };
    
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
    
    let (route, code) = PacketCodeType::MiniGameActionInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}