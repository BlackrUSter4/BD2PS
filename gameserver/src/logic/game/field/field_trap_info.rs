use bd2::prost::Message;
use bd2::proto::proto_net::{FieldTrapDbInfo, FieldTrapInfoRequest, FieldTrapInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::field::field_trap_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account trap state. Judgment call: `FieldTrapInfo.SwitchObjectId` is a single
/// column (not a list) despite the proto's `repeated int32`, a pre-existing schema
/// granularity gap — returned as a one-element list rather than widening the schema for a
/// read-only info request.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FieldTrapInfoRequest) -> GameResponse {
    info!("Handling FieldTrapInfoRequest: {:?}", req);

    let rows = db::get_field_trap_info(pool, uid).await.unwrap_or_default();
    let field_trap_db_info = rows
        .into_iter()
        .filter(|r| req.pack_id.is_none() || r.pack_id == req.pack_id)
        .filter(|r| req.map_id.is_none() || r.map_id == req.map_id)
        .map(|r| FieldTrapDbInfo {
            pack_id: r.pack_id,
            map_id: r.map_id,
            trap_id: r.trap_id,
            state: r.state,
            switch_object_id: vec![r.switch_object_id],
        })
        .collect();

    let response = FieldTrapInfoResponse { field_trap_db_info };

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

    let (route, code) = PacketCodeType::FieldTrapInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
