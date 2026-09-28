use bd2::prost::Message;
use bd2::proto::proto_net::{IbItemDbInfo, IbItemUpgradeRequest, IbItemUpgradeResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::ib::{ib_inventory, ib_play_state};
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, ITEM_UPGRADE_COIN_PER_LEVEL};

pub async fn handle(pool: &SqlitePool, uid: i64, req: IbItemUpgradeRequest) -> GameResponse {
    info!("Handling IbItemUpgradeRequest: {:?}", req);

    let inven_index = req.inven_index.unwrap_or(0);
    let upgrade_count = req.upgrade_count.unwrap_or(1).max(1);
    let item = ib_inventory::get(pool, uid, inven_index).await.ok().flatten();

    let (item_info, total_price) = match item {
        Some(mut item) => {
            let price = upgrade_count * ITEM_UPGRADE_COIN_PER_LEVEL;
            item.level += upgrade_count;
            let _ = ib_inventory::set_level(pool, uid, inven_index, item.level).await;
            let _ = ib_play_state::add_coin(pool, uid, -price).await;
            (
                Some(IbItemDbInfo {
                    inven_index: Some(item.inven_index),
                    r#type: Some(item.r#type),
                    id: Some(item.item_id),
                    level: Some(item.level),
                }),
                price,
            )
        }
        None => (None, 0),
    };

    let state = ib_play_state::get_or_create(pool, uid).await.unwrap_or_default();

    let response = IbItemUpgradeResponse {
        item_info,
        total_upgrade_price: Some(total_price),
        current_coin: Some(state.coin),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::IbItemUpgrade.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
