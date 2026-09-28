use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameSichuanEndRequest, MiniGameSichuanEndResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{mini::{mini_game_sichuan_rank_info as rank_db, mini_game_sichuan_schedule_info as sched_db}, user::user_info as user_db};
use sqlx::SqlitePool;
use tracing::info;

/// Real best-record + real cross-account rank tracking. No sichuan-specific reward table was
/// identified within this pass's scope, so reward_info_bundle stays honestly empty.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameSichuanEndRequest) -> GameResponse {
    info!("Handling MiniGameSichuanEndRequest: {:?}", req);

    let score = req.score.unwrap_or(0) as f64;
    let user = user_db::get_user_info(pool, uid).await.unwrap_or_default().into_iter().next();
    let owner_index = user.as_ref().and_then(|u| u.owner_index).unwrap_or(0);
    let user_id = user.as_ref().and_then(|u| u.user_id.clone()).unwrap_or_default();

    let event_schedule_id = req.game_id.unwrap_or(0);
    let _ = sched_db::upsert_best(pool, uid, event_schedule_id, owner_index, &user_id, score).await;
    let _ = rank_db::upsert_best(pool, uid, owner_index, &user_id, score).await;

    let sched = sched_db::get_by_schedule(pool, uid, event_schedule_id).await.ok().flatten();
    let rank = rank_db::get_top(pool, 100000).await.unwrap_or_default().iter().position(|r| r.uid == uid).map(|p| p as i32 + 1);

    let response = MiniGameSichuanEndResponse {
        reward_info_bundle: None,
        world_best_record_owner_index: sched.as_ref().and_then(|s| s.world_best_record_owner_index),
        world_best_record_user_id: sched.as_ref().and_then(|s| s.world_best_record_user_id.clone()),
        world_best_record_value: sched.as_ref().and_then(|s| s.world_best_record_value),
        rank,
        is_block: req.is_block,
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
    
    let (route, code) = PacketCodeType::MiniGameSichuanEnd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}