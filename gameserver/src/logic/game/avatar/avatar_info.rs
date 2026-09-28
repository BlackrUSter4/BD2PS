use bd2::proto::proto_net::{AvatarInfoRequest, AvatarInfoResponse, AvatarUseDbInfo, ItemDbInfo, Notify};
use bd2::prost::Message;
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::avatar::{avatar_item_info, avatar_use_info};
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: AvatarInfoRequest) -> GameResponse {
    info!("Handling AvatarInfoRequest: {:?}", req);

    let use_info = avatar_use_info::get_or_default(pool, uid).await;
    let avatar_use_info = AvatarUseDbInfo {
        use_char_id: use_info.use_char_id,
        use_hair_id: use_info.use_hair_id,
        use_hair_accessory_id: use_info.use_hair_accessory_id,
        use_face_accessory_id: use_info.use_face_accessory_id,
        use_costume_id: use_info.use_costume_id,
        use_body_accessory_id: use_info.use_body_accessory_id,
        use_hand_accessory_id: use_info.use_hand_accessory_id,
        use_pet_id: use_info.use_pet_id,
        use_mount_id: use_info.use_mount_id,
        use_effect_id: use_info.use_effect_id,
        date: use_info.date,
    };

    // AvatarInfoResponse.item_info is a generic ItemDBInfo list, not the dedicated
    // AvatarItemDBInfo type — that type exists in the schema but is never referenced by any
    // message field anywhere, so we re-encode our owned-avatar-item rows into the generic
    // shape (id = item_id, type = item_category) instead.
    let owned_items = avatar_item_info::get_all(pool, uid).await.unwrap_or_default();
    let item_info = owned_items
        .into_iter()
        .map(|row| ItemDbInfo {
            id: Some(row.item_id),
            r#type: Some(row.item_category),
            count: Some(1),
            ..Default::default()
        })
        .collect();

    let response = AvatarInfoResponse {
        avatar_use_info: Some(avatar_use_info),
        item_info,
    };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::AvatarInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
