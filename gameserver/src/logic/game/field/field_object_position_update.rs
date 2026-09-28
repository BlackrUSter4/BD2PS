use bd2::prost::Message;
use bd2::proto::proto_net::{
    FieldObjectPositionUpdateRequest, FieldObjectPositionUpdateResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::field::{field_object_info, field_object_position_info};
use database::models::game::field::field_object_position_info::FieldObjectPositionInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Real save of a field object's position. Judgment call: `req.pack_id`/`req.group_id`
/// aren't stored (the existing `FieldObjectInfo` schema only keys by `Id`, same
/// pack-scoping gap as `field_object_info.rs`) — the position itself is genuinely
/// persisted and re-readable via FieldObjectInfoRequest afterward.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: FieldObjectPositionUpdateRequest,
) -> GameResponse {
    info!("Handling FieldObjectPositionUpdateRequest: {:?}", req);

    if let (Some(object_id), Some(pos)) = (req.object_id, &req.position) {
        let position = FieldObjectPositionInfo {
            index: 0,
            uid,
            map_id: pos.map_id,
            x: pos.x,
            y: pos.y,
            z: pos.z,
        };
        if let Ok(position_index) = field_object_position_info::insert(pool, &position).await {
            let _ = field_object_info::upsert(pool, uid, object_id, position_index).await;
        }
    }

    let response = FieldObjectPositionUpdateResponse {};

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

    let (route, code) = PacketCodeType::FieldObjectPositionUpdate.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
