use bd2::proto::proto_net::{AvatarShopBuyRequest, AvatarShopBuyResponse, ItemDbInfo, Notify, RewardDbInfoBundle};
use bd2::prost::Message;
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::avatar::avatar_item_info;
use sqlx::SqlitePool;
use tracing::info;

use super::{try_consume_items, AVATAR_ITEM_CATEGORY_PLACEHOLDER};

/// No `AvatarShopTable` data exists to map a `shop_id` to a real (element_type, element_id,
/// element_count) reward — `shop_id` is treated as the avatar item id to unlock directly,
/// under a placeholder category, which is the closest honest interpretation available.
pub async fn handle(pool: &SqlitePool, uid: i64, req: AvatarShopBuyRequest) -> GameResponse {
    info!("Handling AvatarShopBuyRequest: {:?}", req);

    let mut granted = Vec::new();

    for entry in &req.avatar_shop_buy_info {
        let Some(shop_id) = entry.shop_id else { continue };

        let items = entry
            .use_item_info
            .as_ref()
            .map(std::slice::from_ref)
            .unwrap_or(&[]);
        if !try_consume_items(pool, uid, items).await {
            continue;
        }

        let _ = avatar_item_info::grant(pool, uid, AVATAR_ITEM_CATEGORY_PLACEHOLDER, shop_id).await;
        granted.push(ItemDbInfo {
            id: Some(shop_id),
            r#type: Some(AVATAR_ITEM_CATEGORY_PLACEHOLDER),
            count: Some(1),
            ..Default::default()
        });
    }

    let reward_info_bundle = if granted.is_empty() {
        None
    } else {
        Some(RewardDbInfoBundle {
            item_info: granted,
            ..Default::default()
        })
    };

    let response = AvatarShopBuyResponse { reward_info_bundle };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::AvatarShopBuy.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
