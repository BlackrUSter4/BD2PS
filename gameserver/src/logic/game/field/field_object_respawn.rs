use bd2::prost::Message;
use bd2::proto::proto_net::{
    FieldObjectRespawnDbInfo, FieldObjectRespawnRequest, FieldObjectRespawnResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::field::field_object_respawn_info as db;
use database::db::user::user_position::get_current_pack_id;
use sqlx::SqlitePool;
use tracing::info;

/// Real respawn-timer persistence per field-object group, using
/// FieldActionObjectGroupTable.regen_sec for the interval — this is the real
/// table for `field_object_group_id` (its own `id` range, e.g. 601-605,
/// matches the request field exactly); FieldMonsterRegenTable (used here
/// until 2026-10-02) is a DIFFERENT table keyed on monster regen ids, not
/// object-group ids, and `.all().first()` on it returned an arbitrary
/// monster's regen time regardless of which group was actually requested.
/// `field_object_group_id` collides across packs like the monster tables
/// (see Fieldmonstertable::pack_id's doc comment), so this is pack-scoped
/// via `get_current_pack_id` the same way. `field_object_info` in the
/// response is left empty — no data source ties a respawned monster back to
/// a specific FieldObjectDBInfo id.
///
/// NOT fixed here, flagged for later: `FieldObjectRespawnInfo` (the DB table
/// this persists into) keys only on (Uid, FieldObjectGroupId) with no pack
/// column, so two different packs' same-numbered group (e.g. both having a
/// group 601) would still share one respawn-timer row per account. Lower
/// urgency than the lookup-table bug just fixed (this is "two packs'
/// cooldowns interfere," not "wrong data entirely"), but a real gap.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FieldObjectRespawnRequest) -> GameResponse {
    info!("Handling FieldObjectRespawnRequest: {:?}", req);

    let mut field_object_respawn_info = Vec::new();
    if let Some(group_id) = req.field_object_group_id {
        let pack_id = get_current_pack_id(pool, uid).await;
        let regen_sec = data::exceldb::get()
            .fieldactionobjectgrouptable
            .get_by_pack(pack_id, group_id)
            .map(|r| r.regen_sec as i64)
            .unwrap_or(300);
        let respawn_time = chrono::Utc::now().timestamp_millis() + regen_sec * 1000;
        let _ = db::upsert(pool, uid, group_id, respawn_time).await;
        field_object_respawn_info.push(FieldObjectRespawnDbInfo {
            field_object_group_id: Some(group_id),
            respawn_time: Some(respawn_time),
        });
    }

    let response = FieldObjectRespawnResponse {
        field_object_info: vec![],
        field_object_respawn_info,
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
    
    let (route, code) = PacketCodeType::FieldObjectRespawn.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}