use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, RootSortIdInfo as RootSortIdInfoProto, RootSortIdInfoRequest, RootSortIdInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::root::root_sort_id_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account read of the previously-unused RootSortIdInfo table. Correctly empty for a
/// fresh account until something writes a custom sort order into it — no write-side request
/// (e.g. a "save sort order" action) was found anywhere in this project's registered routes, so
/// this table currently has no producer; documented rather than fabricated.
pub async fn handle(pool: &SqlitePool, uid: i64, req: RootSortIdInfoRequest) -> GameResponse {
    info!("Handling RootSortIdInfoRequest: {:?}", req);

    let root_sort_info = db::get_root_sort_id_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| RootSortIdInfoProto { r#type: r.r#type, id: r.id, sort_id: r.sort_id })
        .collect();

    let response = RootSortIdInfoResponse { root_sort_info };

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

    let (route, code) = PacketCodeType::RootSortIdInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
