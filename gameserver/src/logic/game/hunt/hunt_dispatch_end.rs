use bd2::prost::Message;
use bd2::proto::proto_net::{HuntDispatchEndRequest, HuntDispatchEndResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::hunt::hunt_dispatch_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, grant_dispatch_rewards};

/// Real dispatch completion: consumes the account's real HuntDispatchInfo row and grants
/// real rewards computed from HuntDispatchTable.
pub async fn handle(pool: &SqlitePool, uid: i64, req: HuntDispatchEndRequest) -> GameResponse {
    info!("Handling HuntDispatchEndRequest: {:?}", req);

    let group_id = req.hunt_dispatch_group_id.unwrap_or(0);
    let id = req.hunt_dispatch_id.unwrap_or(0);

    let mut reward_info_bundle = None;
    if let Some(row) = db::get_by_group_and_id(pool, uid, group_id, id).await.ok().flatten() {
        if let Some(dispatch) = data::exceldb::get().huntdispatchtable.get(id) {
            reward_info_bundle = Some(grant_dispatch_rewards(pool, uid, dispatch, row.count.unwrap_or(1)).await);
        }
        let _ = db::delete_by_index(pool, uid, row.index).await;
    }

    let response = HuntDispatchEndResponse { reward_info_bundle };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::HuntDispatchEnd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
