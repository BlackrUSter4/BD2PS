use super::now_ms;
use bd2::prost::Message;
use bd2::proto::proto_net::{LifeShopBuyDbInfo, LifeShopBuyInfoRequest, LifeShopBuyInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeShopBuyInfoRequest) -> GameResponse {
    info!("Handling LifeShopBuyInfoRequest: {:?}", req);

    let now = now_ms();
    let rolled_over = database::db::life::life_shop_reset_info::check_and_advance(pool, uid, now)
        .await
        .unwrap_or(false);
    if rolled_over {
        let _ = database::db::life::life_shop_buy_info::reset_all(pool, uid).await;
    }

    let reset = database::db::life::life_shop_reset_info::get(pool, uid)
        .await
        .ok()
        .flatten();
    let buy_rows = database::db::life::life_shop_buy_info::get_by_uid(pool, uid)
        .await
        .unwrap_or_default();

    let response = LifeShopBuyInfoResponse {
        buy_info: buy_rows
            .iter()
            .map(|r| LifeShopBuyDbInfo {
                group_id: Some(r.group_id),
                id: Some(r.shop_id),
                buy_count: Some(r.buy_count),
            })
            .collect(),
        daily_reset_time: reset.as_ref().and_then(|r| r.daily_reset_time),
        weekly_reset_time: reset.as_ref().and_then(|r| r.weekly_reset_time),
        monthly_reset_time: reset.as_ref().and_then(|r| r.monthly_reset_time),
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::LifeShopBuyInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
