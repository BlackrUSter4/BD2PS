use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameActionEndRequest, MiniGameActionEndResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{mini::mini_game_action_single_rank_info as rank_db, user::user_info as user_db};
use sqlx::SqlitePool;
use tracing::info;

/// Real client-reported score recorded into the account's real rank row (only updated if it's
/// a new personal best). No reward-per-score master table was identified within this pass's
/// scope, so reward_info_bundle/new_clear_mission_id stay honestly empty.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameActionEndRequest) -> GameResponse {
    info!("Handling MiniGameActionEndRequest: {:?}", req);

    let score = req.score.unwrap_or(0) as f64;
    let user = user_db::get_user_info(pool, uid).await.unwrap_or_default().into_iter().next();
    let owner_index = user.as_ref().and_then(|u| u.owner_index).unwrap_or(0);
    let user_id = user.as_ref().and_then(|u| u.user_id.clone()).unwrap_or_default();
    let _ = rank_db::upsert_best(pool, uid, owner_index, &user_id, score, req.action_monster.unwrap_or(0)).await;
    let my_best_record = rank_db::get_own(pool, uid).await.ok().flatten().and_then(|r| r.score).unwrap_or(0.0) as i64;

    let response = MiniGameActionEndResponse {
        my_best_record: Some(my_best_record),
        new_clear_mission_id: vec![],
        reward_info_bundle: None,
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
    
    let (route, code) = PacketCodeType::MiniGameActionEnd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}