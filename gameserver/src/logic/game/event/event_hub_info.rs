use bd2::prost::Message;
use bd2::proto::proto_net::{
    EventHubDbInfo, EventHubInfoRequest, EventHubInfoResponse, EventHubSettingDbInfo, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::event::{event_hub_info as hub_db, event_hub_setting_info as setting_db};
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account hub layout. Legitimately empty until something writes an EventHubInfo
/// row for this account — no master "which hubs exist" table was captured (PackEventHubTable
/// is a different, already-used table from the Pack cluster), so there is no server-side
/// source to synthesize a starting layout from.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EventHubInfoRequest) -> GameResponse {
    info!("Handling EventHubInfoRequest: {:?}", req);

    let mut event_hub_info = Vec::new();
    for hub in hub_db::get_event_hub_info(pool, uid).await.unwrap_or_default() {
        let settings = setting_db::get_by_hub_info_index(pool, uid, hub.index)
            .await
            .unwrap_or_default();

        let mut by_slot: std::collections::BTreeMap<(i32, i32), Vec<i32>> =
            std::collections::BTreeMap::new();
        for s in &settings {
            by_slot
                .entry((s.slot.unwrap_or(0), s.hub_content_type.unwrap_or(0)))
                .or_default()
                .push(s.event_uid);
        }

        let setting_info = by_slot
            .into_iter()
            .map(|((slot, hub_content_type), event_uid)| EventHubSettingDbInfo {
                slot: Some(slot),
                hub_content_type: Some(hub_content_type),
                event_uid,
            })
            .collect();

        event_hub_info.push(EventHubDbInfo {
            setting_info,
            uid: Some(hub.index as i32),
            hub_id: hub.hub_id,
            start_time: hub.start_time,
            play_end_time: hub.play_end_time,
            end_time: hub.end_time,
        });
    }

    let response = EventHubInfoResponse {
        event_hub_info,
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

    let (route, code) = PacketCodeType::EventHubInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
