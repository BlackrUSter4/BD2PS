use super::{collection_row_to_dbinfo, now_ms, placeholder_catch, level_for_exp, CATCH_EXP};
use bd2::prost::Message;
use bd2::proto::proto_net::{FishingBiteEndRequest, FishingBiteEndResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Finalizes a catch started by FishingBiteStart/Cheat: grants the fish that was being
/// fought (falling back to a fresh placeholder catch if no session is found, e.g. after a
/// server restart mid-bite) into inventory, updates the collection log, and grants a flat
/// placeholder exp amount (`CATCH_EXP` — no growth-curve table captured).
pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingBiteEndRequest) -> GameResponse {
    info!("Handling FishingBiteEndRequest: {:?}", req);

    let now = now_ms();
    let session = database::db::fishing::fishing_bite_session::get(pool, uid)
        .await
        .ok()
        .flatten();
    let (fish_id, size) = match session {
        Some(s) => (s.fish_id, s.size),
        None => placeholder_catch(None),
    };
    let _ = database::db::fishing::fishing_bite_session::clear(pool, uid).await;

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

    let response = FishingBiteEndResponse {
        reward_info: None,
        collection_fish_info: collection.as_ref().map(collection_row_to_dbinfo),
        level: Some(new_level),
        exp: Some(new_exp),
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
    let (route, code) = PacketCodeType::FishingBiteEnd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
