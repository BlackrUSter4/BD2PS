use bd2::prost::Message;
use bd2::proto::proto_net::{InnOpenRequest, InnOpenResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::reputation::reputation_info as rep_db;
use sqlx::SqlitePool;
use tracing::info;

/// Same real reputation_state pattern as shop_open (fixed group_id=1 placeholder — no per-
/// npc/venue reputation grouping is captured anywhere).
pub async fn handle(pool: &SqlitePool, uid: i64, req: InnOpenRequest) -> GameResponse {
    info!("Handling InnOpenRequest: {:?}", req);

    let reputation_state = rep_db::get_reputation_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|r| r.group_id == Some(1))
        .and_then(|r| r.state);

    let response = InnOpenResponse { reputation_state };
    
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
    
    let (route, code) = PacketCodeType::InnOpen.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}