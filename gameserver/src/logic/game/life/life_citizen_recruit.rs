use super::{citizen_row_to_dbinfo, now_ms};
use bd2::prost::Message;
use bd2::proto::proto_net::{LifeCitizenRecruitRequest, LifeCitizenRecruitResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::models::game::life::life_citizen_info::LifeCitizenInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Recruit a new citizen into a housing slot. The client doesn't attach any item cost to this
/// request (recruiting is presumably gated by having built the housing object itself, tracked
/// separately in LifeWorldObjectInfo) — so this just persists the new citizen as sent.
pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeCitizenRecruitRequest) -> GameResponse {
    info!("Handling LifeCitizenRecruitRequest: {:?}", req);

    let mut result_row = None;

    if let Some(citizen) = &req.citizen_info {
        let avatar = citizen.avatar_info.clone().unwrap_or_default();
        let mut row = LifeCitizenInfo {
            index: 0,
            uid,
            citizen_index: citizen.citizen_index,
            citizen_slot_id: citizen.citizen_slot_id,
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
        if let Ok(new_index) = database::db::life::life_citizen_info::insert(pool, &row).await {
            row.index = new_index;
            result_row = Some(row);
        }
    }

    let response = LifeCitizenRecruitResponse {
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

    let (route, code) = PacketCodeType::LifeCitizenRecruit.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
