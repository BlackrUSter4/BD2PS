use super::try_consume_items;
use bd2::prost::Message;
use bd2::proto::proto_net::{LifeShopBuyRequest, LifeShopBuyResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Consumes the claimed payment and tracks the purchase count for real (so ShopBuyInfo's
/// per-item limits stay consistent). The purchased item itself is unknown without
/// `LifeShopTable`/`LifeBuyItemTable` (not captured), so the reward bundle is left empty.
pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeShopBuyRequest) -> GameResponse {
    info!("Handling LifeShopBuyRequest: {:?}", req);

    let paid = try_consume_items(pool, uid, &req.use_item_info).await;
    if paid {
        if let (Some(group_id), Some(id)) = (req.group_id, req.id) {
            let amount = req.buy_count.unwrap_or(1);
            let _ = database::db::life::life_shop_buy_info::increment_buy_count(
                pool, uid, group_id, id, amount,
            )
            .await;
        }
    }

    let response = LifeShopBuyResponse { reward_info: None };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::LifeShopBuy.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
