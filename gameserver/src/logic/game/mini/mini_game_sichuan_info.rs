use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameSichuanInfoRequest, MiniGameSichuanInfoResponse, MiniGameSichuanScheduleInfo as MiniGameSichuanScheduleInfoProto, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_sichuan_schedule_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real best-record/world-record read. The nested board-layout/reward/active-tile sub-lists
/// (`info`/`reward_info`/`active_info`, each referencing further indexed tables) are left
/// honestly empty this pass — joining them was judged not worth the added complexity relative
/// to the real record-tracking already implemented here.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameSichuanInfoRequest) -> GameResponse {
    info!("Handling MiniGameSichuanInfoRequest: {:?}", req);

    let rows = db::get_mini_game_sichuan_schedule_info(pool, uid).await.unwrap_or_default();
    let info = rows
        .into_iter()
        .filter(|r| req.event_schedule_id.is_empty() || r.event_schedule_id.map(|id| req.event_schedule_id.contains(&id)).unwrap_or(false))
        .map(|r| MiniGameSichuanScheduleInfoProto {
            event_schedule_id: r.event_schedule_id,
            info: vec![],
            world_best_record_owner_index: r.world_best_record_owner_index,
            world_best_record_user_id: r.world_best_record_user_id,
            world_best_record_value: r.world_best_record_value,
            best_record_value: r.best_record_value,
            reward_info: vec![],
            active_info: vec![],
            is_block: r.is_block.map(|v| v != 0),
        })
        .collect();

    let response = MiniGameSichuanInfoResponse { info };
    
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
    
    let (route, code) = PacketCodeType::MiniGameSichuanInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}