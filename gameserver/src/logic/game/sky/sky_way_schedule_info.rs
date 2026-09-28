use bd2::prost::Message;
use bd2::proto::proto_net::{
    Notify, SkyWayScheduleDbInfo, SkyWayScheduleInfoRequest, SkyWayScheduleInfoResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::sky::sky_way_schedule_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account schedule read (was hardcoding the same 7-group fake schedule for every
/// account before, despite SkyWayScheduleInfo already being a real per-account table).
/// Legitimately empty until something writes rows for this account — no SkyWayScheduleTable
/// master data exists to seed a default from.
pub async fn handle(pool: &SqlitePool, uid: i64, req: SkyWayScheduleInfoRequest) -> GameResponse {
    info!("Handling SkyWayScheduleInfoRequest: {:?}", req);

    let rows = db::get_sky_way_schedule_info(pool, uid).await.unwrap_or_default();
    let mut by_group: std::collections::BTreeMap<i32, (Vec<i32>, Option<i32>)> = std::collections::BTreeMap::new();
    for r in rows {
        let entry = by_group.entry(r.group_id.unwrap_or(0)).or_insert((vec![], r.bonus_rate));
        entry.0.push(r.day);
    }

    let schedule_info = by_group
        .into_iter()
        .map(|(group_id, (day, bonus_rate))| SkyWayScheduleDbInfo { day, group_id: Some(group_id), bonus_rate })
        .collect();

    let response = SkyWayScheduleInfoResponse { schedule_info };

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

    let (route, code) = PacketCodeType::SkyWayScheduleInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
