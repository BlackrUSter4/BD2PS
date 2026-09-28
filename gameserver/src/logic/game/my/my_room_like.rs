use bd2::prost::Message;
use bd2::proto::proto_net::{MyRoomLikeRequest, MyRoomLikeResponse, Notify};
use chrono::{Datelike, TimeZone, Utc};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::my::{my_like_info, my_room_user_info};
use sqlx::SqlitePool;
use tracing::info;

/// Real: dedupes one like per (liker, target) per real-time month via `MyLikeInfo.Date`,
/// updates the target's real like count/date on `MyRoomUserInfo`.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MyRoomLikeRequest) -> GameResponse {
    info!("Handling MyRoomLikeRequest: {:?}", req);

    let now = Utc::now();
    let month_start = Utc
        .with_ymd_and_hms(now.year(), now.month(), 1, 0, 0, 0)
        .unwrap()
        .timestamp_millis();
    let now_ms = now.timestamp_millis();

    let (mut monthly_count, mut total_count) = (0i32, 0i32);
    if let Some(target) = req.target_owner_index {
        let already_this_month = my_like_info::get_by_uid_and_target(pool, uid, target)
            .await
            .ok()
            .flatten()
            .map(|row| row.date.unwrap_or(0) >= month_start)
            .unwrap_or(false);

        if !already_this_month {
            let _ = my_like_info::upsert_like(pool, uid, target, now_ms).await;
            let new_total = my_like_info::count_for_target(pool, target)
                .await
                .unwrap_or(0) as i32;
            let _ = my_room_user_info::update_like(pool, target, new_total, now_ms).await;
        }

        total_count = my_like_info::count_for_target(pool, target)
            .await
            .unwrap_or(0) as i32;
        monthly_count = my_like_info::count_for_target_since(pool, target, month_start)
            .await
            .unwrap_or(0) as i32;
    }

    let response = MyRoomLikeResponse {
        monthly_count: Some(monthly_count),
        total_count: Some(total_count),
        date: Some(now_ms),
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
    let (route, code) = PacketCodeType::MyRoomLike.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
