use bd2::proto::proto_net::{AvatarSaveRequest, AvatarSaveResponse, Notify};
use bd2::prost::Message;
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::avatar::avatar_use_info;
use database::models::game::avatar::avatar_use_info::AvatarUseInfo;
use sqlx::SqlitePool;
use tracing::info;

/// No `AvatarItemTable`/`AvatarSetTable` data exists to validate ownership of whatever the
/// client equips into each slot against — accepted unconditionally, same judgment call as
/// e.g. Fishing's boat/map purchases when no cost/ownership table exists to check against.
pub async fn handle(pool: &SqlitePool, uid: i64, req: AvatarSaveRequest) -> GameResponse {
    info!("Handling AvatarSaveRequest: {:?}", req);

    let incoming = req.avatar_use_info.unwrap_or_default();
    let info_row = AvatarUseInfo {
        uid,
        use_char_id: incoming.use_char_id,
        use_hair_id: incoming.use_hair_id,
        use_hair_accessory_id: incoming.use_hair_accessory_id,
        use_face_accessory_id: incoming.use_face_accessory_id,
        use_costume_id: incoming.use_costume_id,
        use_body_accessory_id: incoming.use_body_accessory_id,
        use_hand_accessory_id: incoming.use_hand_accessory_id,
        use_pet_id: incoming.use_pet_id,
        use_mount_id: incoming.use_mount_id,
        use_effect_id: incoming.use_effect_id,
        date: incoming.date.or(Some(chrono::Utc::now().timestamp_millis())),
    };
    let _ = avatar_use_info::save(pool, &info_row).await;

    let response = AvatarSaveResponse {};
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::AvatarSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
