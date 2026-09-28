use bd2::prost::Message;
use bd2::proto::proto_net::{HuntDispatchRewardPreviewRequest, HuntDispatchRewardPreviewResponse, ItemDbInfo, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::hunt::hunt_dispatch_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real preview (no granting) of HuntDispatchTable's real visualItemId/visualItemType reward,
/// scaled by the account's currently in-flight dispatch count for this group/id if one exists.
pub async fn handle(pool: &SqlitePool, uid: i64, req: HuntDispatchRewardPreviewRequest) -> GameResponse {
    info!("Handling HuntDispatchRewardPreviewRequest: {:?}", req);

    let group_id = req.hunt_dispatch_group_id.unwrap_or(0);
    let id = req.hunt_dispatch_id.unwrap_or(0);

    let current_play_count = db::get_by_group_and_id(pool, uid, group_id, id)
        .await
        .ok()
        .flatten()
        .and_then(|r| r.count);

    let mut item_info = Vec::new();
    if let Some(dispatch) = data::exceldb::get().huntdispatchtable.get(id) {
        let rate = dispatch.reward_growth_rate.unwrap_or(1).max(1);
        let count = current_play_count.unwrap_or(1).max(1);
        let per_item = rate * count;
        for i in 0..dispatch.visual_item_id.len() {
            let item_id = dispatch.visual_item_id[i];
            let ty = *dispatch.visual_item_type.get(i).unwrap_or(&1);
            item_info.push(ItemDbInfo { id: Some(item_id), r#type: Some(ty), count: Some(per_item), ..Default::default() });
        }
    }

    let response = HuntDispatchRewardPreviewResponse {
        current_play_count,
        reward_info_bundle: Some(RewardDbInfoBundle { item_info, ..Default::default() }),
        is_not_return_free_ap: Some(false),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::HuntDispatchRewardPreview.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
