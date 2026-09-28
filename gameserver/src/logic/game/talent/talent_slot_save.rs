use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, TalentSlotSaveRequest, TalentSlotSaveResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::talent::talent_object_info as db;
use database::models::game::talent::talent_object_info::TalentObjectInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Judgment call: no dedicated "talent slot" table exists, only the already-scaffolded
/// TalentObjectInfo (talent_type/object_id). Reused it to persist the requested char_id list,
/// one row per slot (talent_type = slot position, object_id = char_id) — the closest real
/// match to "an ordered list of char ids" this schema offers.
pub async fn handle(pool: &SqlitePool, uid: i64, req: TalentSlotSaveRequest) -> GameResponse {
    info!("Handling TalentSlotSaveRequest: {:?}", req);

    let _ = db::delete_talent_object_info(pool, uid).await;
    for (slot, char_id) in req.char_id.iter().enumerate() {
        let _ = db::add_talent_object_info(
            pool,
            &TalentObjectInfo { index: 0, uid, talent_type: Some(slot as i32), object_id: Some(*char_id) },
        )
        .await;
    }

    let response = TalentSlotSaveResponse {};

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

    let (route, code) = PacketCodeType::TalentSlotSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
