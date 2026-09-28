use bd2::prost::Message;
use bd2::proto::proto_net::{
    AttendanceAlwaysInfo, AttendanceEventRewardObtainInfo, AttendanceInfoRequest,
    AttendanceInfoResponse, AttendanceLimitInfo, AttendanceRequest, AttendanceResponse, Notify,
    SubscribeAttendanceInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::attendance::attendance_always_info::get_attendance_always_info;
use database::db::attendance::attendance_event_reward_obtain_info::get_attendance_event_reward_obtain_info;
use database::db::attendance::attendance_info::get_attendance_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: AttendanceRequest) -> GameResponse {
    info!("Handling AttendanceRequest: {:?}", req);

    let always_info = get_attendance_always_info(pool, uid)
        .await
        .unwrap_or_default();

    let event_rewards = get_attendance_event_reward_obtain_info(pool, uid)
        .await
        .unwrap_or_default();

    let subscribe_info: Vec<SubscribeAttendanceInfo> = vec![];
    let _limit_info: Vec<AttendanceLimitInfo> = vec![];

    // --- Map DB models → Protobuf types ---
    let attendance_always_info = always_info
        .into_iter()
        .map(|a| AttendanceAlwaysInfo {
            event_schedule_id: a.event_schedule_id,
            attendance_group_id: a.attendance_group_id,
            attendance_count: a.attendance_count,
        })
        .collect::<Vec<_>>();

    let attendance_event_reward_obtain_info = event_rewards
        .into_iter()
        .map(|r| AttendanceEventRewardObtainInfo {
            event_schedule_id: r.event_schedule_id,
            group_id: r.group_id,
            id: r.id,
        })
        .collect::<Vec<_>>();

    let subscribe_attendance_info = subscribe_info
        .into_iter()
        .map(|s| SubscribeAttendanceInfo {
            ticket_id: s.ticket_id,
            reserved_date: s.reserved_date,
            expiry_date: s.expiry_date,
        })
        .collect::<Vec<_>>();

    let response = AttendanceResponse {
        attendance_always_info,
        attendance_limit_info: vec![], // currently unused
        standard_attendance_info: vec![],
        premium_attendance_info: vec![],
        monthly1_attendance_info: vec![],
        monthly2_attendance_info: vec![],
        subscribe_attendance_info,
        attendance_package_info: vec![],
        attendance_event_reward_obtain_info,
        ..Default::default()
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::Common.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}

pub async fn handle_info(pool: &SqlitePool, uid: i64, req: AttendanceInfoRequest) -> GameResponse {
    info!("Handling AttendanceInfoRequest: {:?}", req);

    let attendance_time = get_attendance_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|a| a.attendance_time)
        .collect::<Vec<_>>();

    let response = AttendanceInfoResponse { attendance_time };
    let resp_bytes = response.encode_to_vec();

    let (route, code) = PacketCodeType::Common.info();
    GameResponse::success(route, &resp_bytes, code)
}
