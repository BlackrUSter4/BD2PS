use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, ProductDbInfo, RewardDbInfoBundle, ShopBuyRequest, ShopBuyResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, shop::shop_product_info as product_db};
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real purchase: consumes the client-reported currency cost (use_item_info) for real, grants
/// the real ProductTable element (element_id/element_type/element_count), and tracks a real
/// per-account buy count against ProductTable's real buy_max_count cap.
pub async fn handle(pool: &SqlitePool, uid: i64, req: ShopBuyRequest) -> GameResponse {
    info!("Handling ShopBuyRequest: {:?}", req);

    let shop_id = req.shop_id.unwrap_or(0);
    let product_id = req.item_info.as_ref().and_then(|i| i.id).unwrap_or(0);

    let mut item_infos = Vec::new();
    let mut product_info = None;

    if let Some(product) = data::exceldb::get().producttable.get(product_id) {
        let already = product_db::get_by_shop(pool, uid, shop_id)
            .await
            .unwrap_or_default()
            .into_iter()
            .find(|p| p.product_id == product_id)
            .map(|p| p.buy_count)
            .unwrap_or(0);

        if already < product.buy_max_count.unwrap_or(i32::MAX) {
            for item in &req.use_item_info {
                if let (Some(id), Some(count)) = (item.id, item.count) {
                    let _ = item_info::consume(pool, uid, id, count).await;
                }
            }

            if let Some(item_id) = product.element_id {
                let count = product.element_count.max(1);
                let _ = item_info::grant(pool, uid, item_id, product.element_type, count).await;
                item_infos.push(ItemDbInfo { id: Some(item_id), r#type: Some(product.element_type), count: Some(count), ..Default::default() });
            }

            let _ = product_db::add_buy_count(pool, uid, shop_id, product_id, 1).await;
            product_info = Some(ProductDbInfo { id: Some(product_id), buy_count: Some(already + 1) });
        }
    }

    let response = ShopBuyResponse {
        reward_info_bundle: Some(RewardDbInfoBundle { item_info: item_infos, ..Default::default() }),
        product_info,
        first_auto_revive_set_char_inven_index: None,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ShopBuy.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
