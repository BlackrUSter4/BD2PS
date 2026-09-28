use bd2::prost::Message;
use bd2::proto::proto_net::{LifeShopSellRequest, LifeShopSellResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Placeholder sell price per unit — real per-item sell values aren't known (no captured
/// ItemTable/LifeSellItemTable), but `life_coin` is a Life-specific balance (a plain field on
/// LifeUserDBInfo, not part of the generic Item/RewardDBInfoBundle system), so at least
/// crediting *something* consistently is more useful than nothing. The client picks up the new
/// balance on its next LifeUserInfo/LifeInfo fetch (RewardDBInfoBundle has no currency field).
const PLACEHOLDER_SELL_PRICE: i32 = 10;

pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeShopSellRequest) -> GameResponse {
    info!("Handling LifeShopSellRequest: {:?}", req);

    let mut total_coin = 0;
    for item in &req.sell_item_info {
        let Some(id) = item.id else { continue };
        let count = item.sell_count.unwrap_or(1);
        if database::db::item::item_info::consume(pool, uid, id, count)
            .await
            .unwrap_or(false)
        {
            total_coin += count * PLACEHOLDER_SELL_PRICE;
        }
    }
    if total_coin > 0 {
        let _ = database::db::life::life_user_info::add_coin(pool, uid, total_coin).await;
    }

    let response = LifeShopSellResponse { reward_info: None };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::LifeShopSell.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
