use bd2::prost::Message;
use bd2::proto::proto_net::{FishingUserInfoRequest, FishingUserInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Views a fishing profile — the caller's own by default, or `target_owner_index` (a real
/// other account's uid, if one exists in this same database) when set.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingUserInfoRequest) -> GameResponse {
    info!("Handling FishingUserInfoRequest: {:?}", req);

    let target_uid = req.target_owner_index.unwrap_or(uid);
    let user = database::db::fishing::fishing_user_info::get(pool, target_uid)
        .await
        .ok()
        .flatten()
        .unwrap_or_default();
    let rod_id = if let Some(idx) = user.use_rod_inven_index {
        database::db::fishing::fishing_rod_info::get_by_index(pool, target_uid, idx)
            .await
            .ok()
            .flatten()
            .map(|r| r.rod_id)
    } else {
        None
    };
    let collection_fish_count = database::db::fishing::fishing_collection_info::get_by_uid(pool, target_uid)
        .await
        .map(|rows| rows.len() as i32)
        .unwrap_or(0);
    let account = database::db::user::user::find_account(pool, target_uid)
        .await
        .ok()
        .flatten();

    let response = FishingUserInfoResponse {
        user_id: account.map(|a| a.user_name),
        exp: Some(user.exp),
        level: Some(user.level),
        rod_id,
        collection_fish_count: Some(collection_fish_count),
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
    let (route, code) = PacketCodeType::FishingUserInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
