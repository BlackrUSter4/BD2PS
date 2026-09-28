use bd2::prost::Message;
use bd2::proto::proto_net::{
    FieldObjectDbInfo, FieldObjectInfoRequest, FieldObjectInfoResponse,
    FieldObjectPositionDbInfo, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::field::{field_object_info, field_object_position_info};
use sqlx::SqlitePool;
use tracing::info;

/// Real persistence of which field objects this account has already interacted with
/// (collected/researched), each with its saved position. Judgment call: the existing
/// scaffolded `FieldObjectInfo` table has no `pack_id` column to scope by field/map, so
/// this returns every saved object for the account rather than filtering by
/// `req.pack_id` — a schema gap inherited from before this stub-audit round, not something
/// introduced here. Everything returned goes under `field_reward_obtain_info`;
/// `field_action_object_info` is left empty since no separate table distinguishes the two.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FieldObjectInfoRequest) -> GameResponse {
    info!("Handling FieldObjectInfoRequest: {:?}", req);

    let rows = field_object_info::get_field_object_info(pool, uid)
        .await
        .unwrap_or_default();

    let mut field_reward_obtain_info = Vec::new();
    for row in rows {
        let position = match row.position_index {
            Some(idx) => field_object_position_info::get_by_index(pool, uid, idx)
                .await
                .ok()
                .map(|p| FieldObjectPositionDbInfo {
                    map_id: p.map_id,
                    x: p.x,
                    y: p.y,
                    z: p.z,
                }),
            None => None,
        };
        field_reward_obtain_info.push(FieldObjectDbInfo {
            id: row.id,
            position,
        });
    }

    let response = FieldObjectInfoResponse {
        field_reward_obtain_info,
        field_action_object_info: vec![],
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

    let (route, code) = PacketCodeType::FieldObjectInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
