use bd2::prost::Message;
use bd2::proto::proto_net::{QuickBattleRequest, QuickBattleResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Generic instant-battle-resolution across an unidentified set of stage-magic modes — no
/// single master table clearly matches `stage_magic_group_id`/`stage_magic_id` combined with
/// `battle_mode`, and no server-side combat resolution exists anywhere in this project (every
/// battle-adjacent system here is client-simulates/server-trusts) — reward stays honestly
/// empty rather than fabricated.
pub async fn handle(_pool: &SqlitePool, _uid: i64, req: QuickBattleRequest) -> GameResponse {
    info!("Handling QuickBattleRequest: {:?}", req);

    let response = QuickBattleResponse { reward_bundle: Some(bd2::proto::proto_net::RewardDbInfoBundle::default()) };
    
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
    
    let (route, code) = PacketCodeType::QuickBattle.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}