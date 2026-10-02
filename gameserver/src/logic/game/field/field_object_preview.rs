use bd2::prost::Message;
use bd2::proto::proto_net::{
    FieldObjectDbInfo, FieldObjectPositionDbInfo, FieldObjectPreviewRequest,
    FieldObjectPreviewResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::field::{field_object_info, field_object_position_info};
use sqlx::SqlitePool;
use tracing::info;

/// FieldResearchObjectTable now carries a real synthesized `pack_id` (2026-10-02,
/// see CLIENT_UPDATE.md), so `research_object_id` is filtered to `req.pack_id` via
/// `all_in_pack` — no longer the cross-pack leak this comment used to describe.
/// The saved-object list is real per-account state, same lookup as FieldObjectInfoRequest.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FieldObjectPreviewRequest) -> GameResponse {
    info!("Handling FieldObjectPreviewRequest: {:?}", req);

    let rows = field_object_info::get_field_object_info(pool, uid).await.unwrap_or_default();
    let mut field_reward_obtain_info = Vec::new();
    for row in rows {
        let position = match row.position_index {
            Some(idx) => field_object_position_info::get_by_index(pool, uid, idx)
                .await
                .ok()
                .map(|p| FieldObjectPositionDbInfo { map_id: p.map_id, x: p.x, y: p.y, z: p.z }),
            None => None,
        };
        field_reward_obtain_info.push(FieldObjectDbInfo { id: row.id, position });
    }

    let research_object_id = req
        .pack_id
        .map(|pack_id| {
            data::exceldb::get()
                .fieldresearchobjecttable
                .all_in_pack(pack_id)
                .map(|r| r.id)
                .collect()
        })
        .unwrap_or_default();

    let response = FieldObjectPreviewResponse {
        field_reward_obtain_info,
        research_object_id,
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
    
    let (route, code) = PacketCodeType::FieldObjectPreview.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}