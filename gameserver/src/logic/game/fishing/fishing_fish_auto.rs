use super::{collection_row_to_dbinfo, now_ms, placeholder_catch, level_for_exp, CATCH_EXP};
use bd2::prost::Message;
use bd2::proto::proto_net::{FishingFishAutoRequest, FishingFishAutoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// One-shot auto-fishing (no bite minigame). Always succeeds — no failure-rate table/formula
/// was captured to roll against, so `is_fishing_success` is deterministically true.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingFishAutoRequest) -> GameResponse {
    info!("Handling FishingFishAutoRequest: {:?}", req);

    let now = now_ms();
    let (fish_id, size) = placeholder_catch(None);
    let _ = database::db::fishing::fishing_fish_info::insert(pool, uid, fish_id, size, now).await;
    let collection = database::db::fishing::fishing_collection_info::record_catch(pool, uid, fish_id, size, now)
        .await
        .ok();

    let user = database::db::fishing::fishing_user_info::get_or_create(pool, uid)
        .await
        .unwrap_or_default();
    let new_exp = user.exp + CATCH_EXP;
    let new_level = level_for_exp(new_exp);
    let _ = database::db::fishing::fishing_user_info::add_exp(pool, uid, CATCH_EXP, new_level).await;

    let response = FishingFishAutoResponse {
        reward_info: None,
        collection_fish_info: collection.as_ref().map(collection_row_to_dbinfo),
        level: Some(new_level),
        exp: Some(new_exp),
        is_fishing_success: Some(true),
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
    let (route, code) = PacketCodeType::FishingFishAuto.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
