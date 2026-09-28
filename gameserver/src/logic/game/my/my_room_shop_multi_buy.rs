use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, MyRoomShopMultiBuyRequest, MyRoomShopMultiBuyResponse, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, my::my_room_shop_info};
use sqlx::SqlitePool;
use tracing::info;

use super::try_consume_items;

/// Same real logic as `MyRoomShopBuy`, applied per item in the batch.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MyRoomShopMultiBuyRequest) -> GameResponse {
    info!("Handling MyRoomShopMultiBuyRequest: {:?}", req);

    let mut granted = Vec::new();

    for entry in &req.item_info {
        let Some(id) = entry.id else { continue };
        let Some(shop_row) = data::exceldb::get().myroomitemshoptable.get(id).cloned() else {
            continue;
        };

        let already_bought = my_room_shop_info::get_my_room_shop_info(pool, uid)
            .await
            .unwrap_or_default()
            .into_iter()
            .find(|r| r.id == Some(id))
            .and_then(|r| r.buy_count)
            .unwrap_or(0);

        let buy_count = entry.item_count.unwrap_or(1).max(1);
        let within_cap = already_bought + buy_count <= shop_row.buy_max_count;

        let items = entry
            .use_item_info
            .as_ref()
            .map(std::slice::from_ref)
            .unwrap_or(&[]);
        let paid = within_cap && try_consume_items(pool, uid, items).await;

        if paid {
            let _ = my_room_shop_info::increment_buy_count(pool, uid, id, buy_count).await;
            let _ = item_info::grant(
                pool,
                uid,
                shop_row.element_id,
                shop_row.element_type,
                shop_row.element_count * buy_count,
            )
            .await;
            granted.push(ItemDbInfo {
                id: Some(shop_row.element_id),
                r#type: Some(shop_row.element_type),
                count: Some(shop_row.element_count * buy_count),
                ..Default::default()
            });
        }
    }

    let reward_info_bundle = if granted.is_empty() {
        None
    } else {
        Some(RewardDbInfoBundle {
            item_info: granted,
            ..Default::default()
        })
    };

    let response = MyRoomShopMultiBuyResponse { reward_info_bundle };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::MyRoomShopMultiBuy.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
