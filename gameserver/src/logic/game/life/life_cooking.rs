use bd2::prost::Message;
use bd2::proto::proto_net::{
    ItemDbInfo, LifeCookingRequest, LifeCookingResponse, Notify, RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

/// CookingTable has no explicit result item type -- 1 (generic item) is this codebase's
/// established default for otherwise-untyped rewards (see GOLD_ITEM_TYPE elsewhere).
const RESULT_ITEM_TYPE: i32 = 1;

/// Real crafting against CookingTable's real material cost / result item, mirroring
/// alchemy::craft's pattern for its own (also real, already-captured) recipe table.
pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeCookingRequest) -> GameResponse {
    info!("Handling LifeCookingRequest: {:?}", req);

    let count = req.count.unwrap_or(1).max(1);
    let recipe = req
        .id
        .and_then(|id| exceldb::get().cookingtable.get(id).cloned());

    let reward_info_bundle = match recipe {
        Some(def) => {
            for i in 0..def.material_item_id.len() {
                let id = def.material_item_id[i];
                let per = *def.material_item_count.get(i).unwrap_or(&1);
                let _ = item_info::consume(pool, uid, id, per * count).await;
            }
            let result_count = def.result_item_count.max(1) * count;
            let _ = item_info::grant(pool, uid, def.result_item_id, RESULT_ITEM_TYPE, result_count)
                .await;
            Some(RewardDbInfoBundle {
                item_info: vec![ItemDbInfo {
                    id: Some(def.result_item_id),
                    r#type: Some(RESULT_ITEM_TYPE),
                    count: Some(result_count),
                    ..Default::default()
                }],
                ..Default::default()
            })
        }
        None => {
            // Unknown recipe id (shouldn't happen with a real client) -- still spend whatever
            // the client claims rather than grant nothing for free.
            let _ = super::try_consume_items(pool, uid, &req.use_item_info).await;
            None
        }
    };

    let response = LifeCookingResponse {
        reward_info_bundle,
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::LifeCooking.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
