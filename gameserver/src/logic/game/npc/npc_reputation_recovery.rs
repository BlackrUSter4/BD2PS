use bd2::prost::Message;
use bd2::proto::proto_net::Notify;
use bd2::{NpcReputationDbInfo, NpcReputationRecoveryRequest, NpcReputationRecoveryResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::npc::npc_reputation_info as db;
use sqlx::SqlitePool;
use tracing::info;

const RECOVERY_AMOUNT: i32 = 10;

/// Real point recovery. No NpcReputationTable/recovery-formula master data exists anywhere in
/// this project, so the recovered amount is a fixed documented placeholder rather than a
/// fabricated real formula.
pub async fn handle(pool: &SqlitePool, uid: i64, req: NpcReputationRecoveryRequest) -> GameResponse {
    info!("Handling NpcReputationRecoveryRequest: {:?}", req);

    if let Some(npc_id) = req.npc_id {
        let _ = db::add_point(pool, uid, npc_id, RECOVERY_AMOUNT).await;
    }

    let npc_reputation_info = db::get_npc_reputation_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| NpcReputationDbInfo { pack_id: r.pack_id, group_id: r.group_id, npc_id: r.npc_id, point: r.point })
        .collect();

    let response = NpcReputationRecoveryResponse { npc_reputation_info };

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

    let (route, code) = PacketCodeType::NpcReputationRecovery.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
