use super::{citizen_row_to_dbinfo, now_ms};
use bd2::prost::Message;
use bd2::proto::proto_net::{LifeCitizenAvatarSaveRequest, LifeCitizenAvatarSaveResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::models::game::life::life_citizen_info::LifeCitizenInfo;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeCitizenAvatarSaveRequest) -> GameResponse {
    info!("Handling LifeCitizenAvatarSaveRequest: {:?}", req);

    let mut result_row = None;

    if let Some(citizen) = &req.citizen_info {
        let slot = citizen.citizen_slot_id.unwrap_or_default();
        let avatar = citizen.avatar_info.clone().unwrap_or_default();

        let mut row = LifeCitizenInfo {
            index: 0,
            uid,
            citizen_index: citizen.citizen_index,
            citizen_slot_id: Some(slot),
            use_char_id: avatar.use_char_id,
            use_hair_id: avatar.use_hair_id,
            use_hair_accessory_id: avatar.use_hair_accessory_id,
            use_face_accessory_id: avatar.use_face_accessory_id,
            use_costume_id: avatar.use_costume_id,
            use_body_accessory_id: avatar.use_body_accessory_id,
            use_hand_accessory_id: avatar.use_hand_accessory_id,
            use_pet_id: avatar.use_pet_id,
            use_mount_id: avatar.use_mount_id,
            use_effect_id: avatar.use_effect_id,
            avatar_date: Some(now_ms()),
        };

        if let Ok(Some(existing)) =
            database::db::life::life_citizen_info::get_by_slot(pool, uid, slot).await
        {
            row.index = existing.index;
            let _ = database::db::life::life_citizen_info::update_avatar(pool, &row).await;
        } else if let Ok(new_index) = database::db::life::life_citizen_info::insert(pool, &row).await
        {
            row.index = new_index;
        }

        result_row = Some(row);
    }

    let response = LifeCitizenAvatarSaveResponse {
        citizen_info: result_row.map(|r| citizen_row_to_dbinfo(&r)),
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

    let (route, code) = PacketCodeType::LifeCitizenAvatarSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
