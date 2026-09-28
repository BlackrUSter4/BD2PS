use bd2::prost::Message;
use bd2::proto::proto_net::{FishingShopSellRequest, FishingShopSellResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// JUDGMENT CALL: `FishingShopSellItemDBInfo` carries both an `inven_index` AND a
/// `group_id`/`id` shop-catalog reference, which is ambiguous — it could mean "sell this
/// caught fish" (by inven_index into FishingFishInfo) or "sell this fishing item" (by
/// inven_index into FishingItemInfo, with group_id/id just echoing the shop catalog entry
/// it was bought from). Went with the item-inventory interpretation since "ShopSellItem"
/// names it as an *item* sell, not a fish sell (there's a separate, unrelated fish-lock flow
/// for fish already). No price table was captured either way, so no currency is granted —
/// only the sold stock is actually removed for real.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FishingShopSellRequest) -> GameResponse {
    info!("Handling FishingShopSellRequest: {:?}", req);

    for sell in &req.sell_item_info {
        if let Some(inven_index) = sell.inven_index {
            let items = database::db::fishing::fishing_item_info::get_by_uid(pool, uid)
                .await
                .unwrap_or_default();
            if let Some(row) = items.iter().find(|r| r.inven_index == inven_index) {
                let count = sell.sell_count.unwrap_or(row.count).min(row.count);
                if count > 0 {
                    let _ = database::db::fishing::fishing_item_info::consume(
                        pool, uid, row.item_id, count,
                    )
                    .await;
                }
            }
        }
    }

    let response = FishingShopSellResponse { reward_info: None };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::FishingShopSell.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
