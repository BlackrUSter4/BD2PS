use bd2::prost::Message;
use bd2::proto::proto_net::{BattleRetryRequest, BattleRetryResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::battle::battle_session;
use rand::Rng;
use sqlx::SqlitePool;
use tracing::info;

/// Retrying the same battle from the start — keeps the existing session
/// (group/monster/pack context from Enter) but issues a fresh real random
/// seed, same as Start.
pub async fn handle(pool: &SqlitePool, uid: i64, req: BattleRetryRequest) -> GameResponse {
    info!("Handling BattleRetryRequest: {:?}", req);

    if let Some(battle_index) = req.battle_index {
        let seed: i32 = rand::thread_rng().gen_range(i32::MIN..=i32::MAX);
        if let Err(e) = battle_session::set_start(pool, uid, battle_index, seed).await {
            tracing::warn!("BattleRetry: failed to update battle session: {}", e);
        }
    }

    let response = BattleRetryResponse {
        ..Default::default()
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

    let (route, code) = PacketCodeType::BattleRetry.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
