use bd2::prost::Message;
use bd2::proto::proto_net::{ProductDbInfo, ShopDbInfo, ShopInfoRequest, ShopInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::shop::{shop_info as shop_db, shop_product_info as product_db};
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, reset_seconds_for};

/// Real per-shop rotation: get-or-refresh the account's ShopInfo row for this shop_id (real
/// reset duration from ShopTable), real product list from ProductTable filtered by
/// group_id == shop_id (judgment call — no explicit shop->product FK exists in the schema,
/// but group_id is the only field that plausibly links them), real per-product buy counts.
pub async fn handle(pool: &SqlitePool, uid: i64, req: ShopInfoRequest) -> GameResponse {
    info!("Handling ShopInfoRequest: {:?}", req);

    let shop_id = req.shop_id.unwrap_or(0);
    let game_data = data::exceldb::get();

    let mut shop_dbinfo = None;
    if let Some(shop) = game_data.shoptable.get(shop_id) {
        let reset_seconds = reset_seconds_for(shop);
        let row = shop_db::get_or_refresh(pool, uid, shop_id, reset_seconds).await.ok();

        if let Some(row) = row {
            let buy_counts = product_db::get_by_shop(pool, uid, shop_id).await.unwrap_or_default();
            let product_info = game_data
                .producttable
                .by_group(shop_id)
                .map(|p| {
                    let buy_count = buy_counts.iter().find(|b| b.product_id == p.id).map(|b| b.buy_count).unwrap_or(0);
                    ProductDbInfo { id: Some(p.id), buy_count: Some(buy_count) }
                })
                .collect();

            shop_dbinfo = Some(ShopDbInfo {
                shop_remain_time: row.shop_remain_time,
                shop_rand_seed: row.shop_rand_seed,
                product_info,
            });
        }
    }

    let response = ShopInfoResponse { shop_info: shop_dbinfo };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ShopInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
