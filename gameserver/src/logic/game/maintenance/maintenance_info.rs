use bd2::prost::Message;
use bd2::proto::proto_net::{MaintenanceInfo, MaintenanceInfoRequest, MaintenanceInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(_pool: &SqlitePool, _uid: i64, req: MaintenanceInfoRequest) -> GameResponse {
    info!("Handling MaintenanceInfoRequest: {:?}", req);

    // Must match gameserver::logic::game::maintenace::maintenace_info (pre-login route) --
    // the client re-sends MaintenanceInfo mid-session and overwrites BDNetwork.CdnInfo.Version
    // with whatever bundle_version comes back here. Leaving this at Default::default() blanks
    // that version out again, which breaks Addressables resolution for any CDN-hosted asset
    // (bundle_version stale/empty causes InvalidKeyException: No Location found).
    let market_info = MaintenanceInfo {
        market_type: Some(4),
        version: Some("2.8.13".to_string()),
        // CORRECTION (2026-09-30, same night): a prior "fix" here changed this to
        // 20260923193640 after observing that value in GameData/ CDN request paths --
        // wrong move. That value belongs to a DIFFERENT version scheme (ServerInfo's
        // separate `game_data_version` field). THIS field's real, correct value was
        // confirmed empirically: https://bd2-cdn.akamaized.net/ServerData/StandaloneWindows64/HD/<value>/catalog_alpha.hash
        // returns 200 for 20260921135230 and 404 for 20260923193640. Do not change this
        // again without testing that exact URL first.
        bundle_version: Some("20260921135230".to_string()),
        is_bundle_update: Some(false),
        maintenance_type: Some(0),
        date: Some("2023-09-18 00-00-00".to_string()),
        region_list: Some("".to_string()),
        use_dsa: Some(1),
        maintenance_url: Some(
            "http://mwe.dq.pmang.com/:brown_Event/maintenance?e=900&market_type=*".to_string(),
        ),
        use_maintenance_url: Some(0),
        download_url: Some("https://www.browndust2.com/".to_string()),
        notice: Some("".to_string()),
        bundle_version_sd: Some("20260921135230".to_string()),
    };

    let response = MaintenanceInfoResponse {
        market_info: Some(market_info),
        ..Default::default()
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

    let (route, code) = PacketCodeType::Common.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
