use bd2::prost::Message;
use bd2::proto::proto_net::{IdCardRecoveryRequest, IdCardRecoveryResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Judgment call: "recovery" is read as restoring the account's currently-saved card (real
/// read), not resetting it — there's no separate "default card" table to reset to, and
/// resetting on the only request that reads it back would be destructive. No reward table
/// exists for this action either, so the bundle is left empty rather than fabricated.
pub async fn handle(pool: &SqlitePool, uid: i64, req: IdCardRecoveryRequest) -> GameResponse {
    info!("Handling IdCardRecoveryRequest: {:?}", req);

    let id_card_info = match database::db::id::id_card_info::get_current(pool, uid).await.ok().flatten() {
        Some(row) => Some(super::card_to_proto(pool, uid, &row).await),
        None => None,
    };

    let response = IdCardRecoveryResponse { reward_info_bundle: None, id_card_info };
    
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
    
    let (route, code) = PacketCodeType::IdCardRecovery.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}