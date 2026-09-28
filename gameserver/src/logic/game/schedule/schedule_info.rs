use bd2::prost::Message;
use bd2::proto::proto_net::{
    Notify, ScheduleDbInfo, ScheduleInfoRequest, ScheduleInfoResponse, SeasonInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::{Row, SqlitePool};
use tracing::{error, info};

pub async fn handle(pool: &SqlitePool, _uid: i64, req: ScheduleInfoRequest) -> GameResponse {
    info!("Handling ScheduleInfoRequest: {:?}", req);

    let rows = match sqlx::query("SELECT * FROM ScheduleInfo ORDER BY ContentId ASC")
        .fetch_all(pool)
        .await
    {
        Ok(r) => r,
        Err(e) => {
            error!("DB error fetching ScheduleInfo: {e}");
            return GameResponse::error(400);
        }
    };

    let mut schedule_infos = Vec::with_capacity(rows.len());

    for row in rows {
        // --- Current Season ---
        let current = SeasonInfo {
            season: Some(row.try_get::<i64, _>("CurrentSeason").unwrap_or_default() as i32),
            start_time: Some(
                row.try_get::<i64, _>("CurrentStartTime")
                    .unwrap_or_default(),
            ),
            end_time: Some(row.try_get::<i64, _>("CurrentEndTime").unwrap_or_default()),
            error_flag: Some(row.try_get::<bool, _>("CurrentErrorFlag").unwrap_or(false)),
            return_flag: Some(row.try_get::<bool, _>("CurrentReturnFlag").unwrap_or(false)),
            rank_reward_group_id: Some(
                row.try_get::<i64, _>("CurrentRankRewardGroupId")
                    .unwrap_or_default() as i32,
            ),
        };

        // --- Next Season ---
        let next = SeasonInfo {
            season: Some(row.try_get::<i64, _>("NextSeason").unwrap_or_default() as i32),
            start_time: Some(row.try_get::<i64, _>("NextStartTime").unwrap_or_default()),
            end_time: Some(row.try_get::<i64, _>("NextEndTime").unwrap_or_default()),
            error_flag: Some(row.try_get::<bool, _>("NextErrorFlag").unwrap_or(false)),
            return_flag: Some(row.try_get::<bool, _>("NextReturnFlag").unwrap_or(false)),
            rank_reward_group_id: Some(
                row.try_get::<i64, _>("NextRankRewardGroupId")
                    .unwrap_or_default() as i32,
            ),
        };

        // --- Whole Schedule Entry ---
        let db_info = ScheduleDbInfo {
            content_id: Some(row.try_get::<i64, _>("ContentId").unwrap_or_default() as i32),
            current_season: Some(current),
            next_season: Some(next),
        };

        schedule_infos.push(db_info);
    }

    let calculate_ms = 32400000;

    // 4️⃣ Assemble final response
    let response = ScheduleInfoResponse {
        schedule_calculate_mile_seconds: Some(calculate_ms),
        schedule_info: schedule_infos,
    };

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

    let (route, code) = PacketCodeType::ScheduleInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
