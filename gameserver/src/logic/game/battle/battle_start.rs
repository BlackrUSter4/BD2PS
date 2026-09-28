use bd2::prost::Message;
use bd2::proto::proto_net::{BattleStartRequest, BattleStartResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::battle::battle_session;
use rand::Rng;
use sqlx::SqlitePool;
use tracing::info;

/// Same async-battle pattern already established for Colosseum/Ib: the
/// client simulates the fight itself against a server-supplied random seed
/// and its own already-known roster, then reports the outcome via
/// BattleEnd. So Start's real job is just handing back a genuine random
/// seed and persisting battle_index (so BattleRetry/VerifyState can find
/// this session again) — the roster is simply echoed back since the
/// request already carries the authoritative one.
pub async fn handle(pool: &SqlitePool, uid: i64, req: BattleStartRequest) -> GameResponse {
    info!("Handling BattleStartRequest: {:?}", req);

    let seed: i32 = rand::thread_rng().gen_range(i32::MIN..=i32::MAX);
    if let Some(battle_index) = req.battle_index {
        if let Err(e) = battle_session::set_start(pool, uid, battle_index, seed).await {
            tracing::warn!("BattleStart: failed to update battle session: {}", e);
        }
    }

    let response = BattleStartResponse {
        red_char_info: req.red_char_info.clone(),
        blue_char_info: req.blue_char_info.clone(),
        battle_random_seed: Some(seed),
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

    let (route, code) = PacketCodeType::BattleStart.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
