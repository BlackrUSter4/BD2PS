use bd2::prost::Message;
use bd2::proto::proto_net::{CostumeBaseDbInfo, SupporterInfoRequest, SupporterInfoResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::supporter::supporter_slot_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real list of this account's own registered supporter costumes. `guild_supporter_info`
/// is left empty — that's a separate guild-scoped table (GuildSupporterInfo) out of scope
/// for this per-account handler.
pub async fn handle(pool: &SqlitePool, uid: i64, req: SupporterInfoRequest) -> GameResponse {
    info!("Handling SupporterInfoRequest: {:?}", req);

    let rows = db::get_supporter_slot_info(pool, uid).await.unwrap_or_default();
    let supporter_info = rows
        .into_iter()
        .filter_map(|r| r.costume_id.map(|id| CostumeBaseDbInfo { id: Some(id), level: None, design_id: None }))
        .collect();

    let response = SupporterInfoResponse { supporter_info, guild_supporter_info: vec![] };
    
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
    
    let (route, code) = PacketCodeType::SupporterInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}