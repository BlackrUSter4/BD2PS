use bd2::prost::Message;
use bd2::proto::proto_net::{HuntDispatchDbInfo, HuntDispatchInfoRequest, HuntDispatchInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::hunt::hunt_dispatch_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real per-account active dispatches. `hunting_ground_min_pack_id` is the real minimum
/// pack_id across all of HuntDispatchTable (no separate "hunting ground unlock" state is
/// tracked anywhere in this project to derive it from instead).
pub async fn handle(pool: &SqlitePool, uid: i64, req: HuntDispatchInfoRequest) -> GameResponse {
    info!("Handling HuntDispatchInfoRequest: {:?}", req);

    let rows = db::get_hunt_dispatch_info(pool, uid).await.unwrap_or_default();
    let hunt_dispatch_info = rows
        .into_iter()
        .map(|r| HuntDispatchDbInfo {
            hunt_dispatch_group_id: r.hunt_dispatch_group_id,
            hunt_dispatch_id: r.hunt_dispatch_id,
            count: r.count,
            start_time: r.start_time,
            end_time: r.end_time,
            decrease_free_ap_count: r.decrease_free_ap_count,
            decrease_cash_ap_count: r.decrease_cash_ap_count,
        })
        .collect();

    let hunting_ground_min_pack_id = data::exceldb::get()
        .huntdispatchtable
        .all()
        .iter()
        .map(|t| t.pack_id)
        .min();

    let response = HuntDispatchInfoResponse { hunt_dispatch_info, hunting_ground_min_pack_id };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::HuntDispatchInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
