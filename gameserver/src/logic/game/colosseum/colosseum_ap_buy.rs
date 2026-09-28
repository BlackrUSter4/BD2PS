use bd2::prost::Message;
use bd2::proto::proto_net::{ColosseumApBuyRequest, ColosseumApBuyResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::colosseum::colosseum_user_info;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, empty_reward_bundle, now_ms, AP_BUY_MAX_PER_DAY, DAY_MS};

pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumApBuyRequest) -> GameResponse {
    info!("Handling ColosseumApBuyRequest: {:?}", req);

    let user = colosseum_user_info::get_or_create(pool, uid).await.unwrap_or_default();
    let now = now_ms();
    let reset_needed = user.ap_buy_reset_time.map(|t| now - t >= DAY_MS).unwrap_or(true);
    let mut count = if reset_needed { 0 } else { user.ap_buy_count };

    if count < AP_BUY_MAX_PER_DAY {
        // Real payment: consume whatever material items the client says it's paying with.
        for item in &req.decrease_item_info {
            if let (Some(id), Some(cnt)) = (item.id, item.count) {
                let _ = database::db::item::item_info::consume(pool, uid, id, cnt).await;
            }
        }
        count += 1;
        let reset_time = if reset_needed { now } else { user.ap_buy_reset_time.unwrap_or(now) };
        let _ = colosseum_user_info::set_ap_buy(pool, uid, count, reset_time).await;
    }

    let response = ColosseumApBuyResponse {
        reward_info_bundle: Some(empty_reward_bundle()),
        ap_buy_count: Some(count),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumApBuy.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
