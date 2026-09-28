use bd2::prost::Message;
use bd2::proto::proto_net::{MonsterDbInfo, Notify, SkyWayEnterRequest, SkyWayEnterResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::sky::sky_way_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real progress update against the account's real SkyWayInfo row. monster_info uses
/// SkyWayFieldTable's real monster_id list; no per-instance spawn table (respawn timers etc)
/// exists for this system, so only the real monster ids are populated.
pub async fn handle(pool: &SqlitePool, uid: i64, req: SkyWayEnterRequest) -> GameResponse {
    info!("Handling SkyWayEnterRequest: {:?}", req);

    let mut monster_info = Vec::new();
    if let (Some(group_id), Some(id)) = (req.group_id, req.id) {
        let _ = db::upsert_progress(pool, uid, group_id, id, req.is_auto).await;

        if let Some(field) = data::exceldb::get().skywayfieldtable.get(id) {
            monster_info = field
                .monster_id
                .iter()
                .map(|&monster_id| MonsterDbInfo { monster_id: Some(monster_id), ..Default::default() })
                .collect();
        }
    }

    let response = SkyWayEnterResponse { monster_info };

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

    let (route, code) = PacketCodeType::SkyWayEnter.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
