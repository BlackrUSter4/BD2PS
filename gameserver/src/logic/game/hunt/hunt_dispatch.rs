use bd2::prost::Message;
use bd2::proto::proto_net::{HuntDispatchRequest, HuntDispatchResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, grant_dispatch_rewards};

/// Real instant dispatch (distinct from the timed Start/End pair): grants real rewards from
/// HuntDispatchTable immediately for the requested count, no waiting row persisted.
pub async fn handle(pool: &SqlitePool, uid: i64, req: HuntDispatchRequest) -> GameResponse {
    info!("Handling HuntDispatchRequest: {:?}", req);

    let group_id = req.hunt_dispatch_group_id.unwrap_or(0);
    let id = req.hunt_dispatch_id.unwrap_or(0);
    let count = req.count.filter(|&c| c > 0).unwrap_or(1);

    let mut reward_info_bundle = None;
    if let Some(dispatch) = data::exceldb::get().huntdispatchtable.get(id) {
        if dispatch.group_id == group_id {
            reward_info_bundle = Some(grant_dispatch_rewards(pool, uid, dispatch, count).await);
        }
    }

    let response = HuntDispatchResponse { reward_info_bundle };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::HuntDispatch.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
