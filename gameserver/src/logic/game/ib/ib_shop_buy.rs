use bd2::prost::Message;
use bd2::proto::proto_net::{IbItemDbInfo, IbShopBuyRequest, IbShopBuyResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::ib::{ib_inventory, ib_play_state, ib_shop};
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(pool: &SqlitePool, uid: i64, req: IbShopBuyRequest) -> GameResponse {
    info!("Handling IbShopBuyRequest: {:?}", req);

    let slot = req.slot.unwrap_or(0);
    let shop_item = ib_shop::get(pool, uid, slot).await.ok().flatten();

    let (item_info, decrease_coin) = match shop_item {
        Some(item) if !item.is_sold_out => {
            let _ = ib_play_state::add_coin(pool, uid, -item.price).await;
            let _ = ib_shop::set_sold_out(pool, uid, slot).await;
            let inven_index = ib_inventory::add_item(pool, uid, item.r#type, item.item_id, 1)
                .await
                .unwrap_or(0);
            (
                Some(IbItemDbInfo {
                    inven_index: Some(inven_index),
                    r#type: Some(item.r#type),
                    id: Some(item.item_id),
                    level: Some(1),
                }),
                item.price,
            )
        }
        // Unknown or already-sold-out slot — degrade gracefully rather than error.
        _ => (None, 0),
    };

    let state = ib_play_state::get_or_create(pool, uid).await.unwrap_or_default();

    let response = IbShopBuyResponse {
        item_info,
        decrease_coin: Some(decrease_coin),
        current_coin: Some(state.coin),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::IbShopBuy.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
