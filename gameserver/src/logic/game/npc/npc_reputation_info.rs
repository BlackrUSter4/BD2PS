use bd2::prost::Message;
use bd2::proto::proto_net::Notify;
use bd2::{NpcReputationDbInfo, NpcReputationInfoRequest, NpcReputationInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::npc::npc_reputation_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account NPC reputation, optionally scoped by group_id.
pub async fn handle(pool: &SqlitePool, uid: i64, req: NpcReputationInfoRequest) -> GameResponse {
    info!("Handling NpcReputationInfoRequest: {:?}", req);

    let rows = db::get_npc_reputation_info(pool, uid).await.unwrap_or_default();
    let npc_reputation_info = rows
        .into_iter()
        .filter(|r| req.group_id.is_none() || r.group_id == req.group_id)
        .map(|r| NpcReputationDbInfo { pack_id: r.pack_id, group_id: r.group_id, npc_id: r.npc_id, point: r.point })
        .collect();

    let response = NpcReputationInfoResponse { npc_reputation_info };

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

    let (route, code) = PacketCodeType::NpcReputationInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
