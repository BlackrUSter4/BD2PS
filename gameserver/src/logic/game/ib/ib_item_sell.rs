use bd2::prost::Message;
use bd2::proto::proto_net::{IbItemSellRequest, IbItemSellResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::ib::{ib_inventory, ib_play_state};
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, ITEM_SELL_COIN_PER_LEVEL};

pub async fn handle(pool: &SqlitePool, uid: i64, req: IbItemSellRequest) -> GameResponse {
    info!("Handling IbItemSellRequest: {:?}", req);

    let inven_index = req.inven_index.unwrap_or(0);
    let item = ib_inventory::get(pool, uid, inven_index).await.ok().flatten();

    let (sell_price, current_coin) = match item {
        Some(item) => {
            let price = item.level.max(1) * ITEM_SELL_COIN_PER_LEVEL;
            let _ = ib_inventory::remove(pool, uid, inven_index).await;
            let coin = ib_play_state::add_coin(pool, uid, price).await.unwrap_or(0);
            (price, coin)
        }
        // Unknown/already-sold item — degrade gracefully rather than error.
        None => {
            let state = ib_play_state::get_or_create(pool, uid).await.unwrap_or_default();
            (0, state.coin)
        }
    };

    let response = IbItemSellResponse {
        inven_index: Some(inven_index as i32),
        sell_price: Some(sell_price),
        current_coin: Some(current_coin),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::IbItemSell.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
