use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, RewardDbInfoBundle, ShopSellRequest, ShopSellResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

const GOLD_ITEM_ID: i32 = 4;
const GOLD_ITEM_TYPE: i32 = 1;

/// Real sell-back: removes the owned item stacks for real, grants real gold using each item's
/// own real `rate` (sell-price multiplier) x count from the client-reported ShopItemDBInfo —
/// no separate server-side sell-price table exists to validate `rate` against, same
/// client-trusted-value precedent as other buy/sell handlers in this project.
pub async fn handle(pool: &SqlitePool, uid: i64, req: ShopSellRequest) -> GameResponse {
    info!("Handling ShopSellRequest: {:?}", req);

    let mut total_gold = 0i32;
    for item in &req.item_info {
        if let (Some(inven_index), Some(count), Some(rate)) = (item.inven_index, item.item_count, item.rate) {
            if item_info::reduce_by_inven_index(pool, uid, inven_index, count).await.unwrap_or(false) {
                total_gold += rate * count;
            }
        }
    }

    if total_gold > 0 {
        let _ = item_info::grant(pool, uid, GOLD_ITEM_ID, GOLD_ITEM_TYPE, total_gold).await;
    }

    let response = ShopSellResponse {
        reward_info_bundle: Some(RewardDbInfoBundle {
            item_info: vec![ItemDbInfo { id: Some(GOLD_ITEM_ID), r#type: Some(GOLD_ITEM_TYPE), count: Some(total_gold), ..Default::default() }],
            ..Default::default()
        }),
    };

    let resp_bytes = response.encode_to_vec();

    let notify = super::default_notify();
    let (route, code) = PacketCodeType::ShopSell.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
