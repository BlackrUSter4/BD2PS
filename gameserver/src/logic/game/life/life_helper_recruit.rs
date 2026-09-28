use super::{helper_row_to_dbinfo, now_ms};
use bd2::prost::Message;
use bd2::proto::proto_net::{LifeHelperRecruitRequest, LifeHelperRecruitResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::models::game::life::life_helper_info::LifeHelperInfo;
use sqlx::SqlitePool;
use tracing::info;

/// No item cost is attached to this request (unlike most other Life actions) — recruiting is
/// presumably gated client-side by `is_trade` / a prior gacha roll, so this just persists the
/// helper as sent.
pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeHelperRecruitRequest) -> GameResponse {
    info!("Handling LifeHelperRecruitRequest: {:?}", req);

    let mut helper_info = Vec::new();

    if let Some(target) = &req.helper_info {
        let avatar = target.avatar_info.clone().unwrap_or_default();
        let mut row = LifeHelperInfo {
            index: 0,
            uid,
            helper_index: target.helper_index,
            helper_id: target.helper_id,
            helper_slot_id: target.helper_slot_id,
            helper_name: target.helper_name.clone(),
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
            work_type: None,
            work_id: None,
            assign_date: None,
        };
        if let Ok(new_index) = database::db::life::life_helper_info::insert(pool, &row).await {
            row.index = new_index;
            helper_info.push(helper_row_to_dbinfo(&row));
        }
    }

    let response = LifeHelperRecruitResponse {
        helper_info,
        reward_bundle: None,
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

    let (route, code) = PacketCodeType::LifeHelperRecruit.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
