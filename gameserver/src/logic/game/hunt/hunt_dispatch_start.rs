use bd2::prost::Message;
use bd2::proto::proto_net::{HuntDispatchDbInfo, HuntDispatchStartRequest, HuntDispatchStartResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::hunt::hunt_dispatch_info as db;
use database::models::game::hunt::hunt_dispatch_info::HuntDispatchInfo;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real timed dispatch start against HuntDispatchTable's real apPerTime/clearTime. No AP/
/// stamina currency column exists anywhere in this project (confirmed missing in every prior
/// cluster's investigation), so the real AP cost is recorded on the row but not actually
/// deducted from anything.
pub async fn handle(pool: &SqlitePool, uid: i64, req: HuntDispatchStartRequest) -> GameResponse {
    info!("Handling HuntDispatchStartRequest: {:?}", req);

    let group_id = req.hunt_dispatch_group_id.unwrap_or(0);
    let id = req.hunt_dispatch_id.unwrap_or(0);
    let count = req.count.filter(|&c| c > 0).unwrap_or(1);

    let mut hunt_dispatch_info = None;
    if let Some(dispatch) = data::exceldb::get().huntdispatchtable.get(id) {
        if dispatch.group_id == group_id {
            let now = chrono::Utc::now().timestamp_millis();
            let end_time = now + (dispatch.clear_time as i64) * count as i64 * 1000;
            let ap_cost = dispatch.ap_per_time * count;

            let row = HuntDispatchInfo {
                index: 0,
                uid,
                hunt_dispatch_group_id: Some(group_id),
                hunt_dispatch_id: Some(id),
                count: Some(count),
                start_time: Some(now),
                end_time: Some(end_time),
                decrease_free_ap_count: Some(ap_cost),
                decrease_cash_ap_count: Some(0),
            };
            let _ = db::add_hunt_dispatch_info(pool, &row).await;

            hunt_dispatch_info = Some(HuntDispatchDbInfo {
                hunt_dispatch_group_id: row.hunt_dispatch_group_id,
                hunt_dispatch_id: row.hunt_dispatch_id,
                count: row.count,
                start_time: row.start_time,
                end_time: row.end_time,
                decrease_free_ap_count: row.decrease_free_ap_count,
                decrease_cash_ap_count: row.decrease_cash_ap_count,
            });
        }
    }

    let response = HuntDispatchStartResponse { hunt_dispatch_info };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::HuntDispatchStart.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
