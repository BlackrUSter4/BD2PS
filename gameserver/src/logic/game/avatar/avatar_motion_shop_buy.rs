use bd2::proto::proto_net::{AvatarMotionShopBuyRequest, AvatarMotionShopBuyResponse, ItemDbInfo, Notify, RewardDbInfoBundle};
use bd2::prost::Message;
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::avatar::avatar_motion_info;
use sqlx::SqlitePool;
use tracing::info;

use super::AVATAR_MOTION_ITEM_TYPE_PLACEHOLDER;

/// The request carries no payment field at all (just `seq` + `shop_id`) and no
/// `AvatarMotionShopTable` data exists to check a cost against even if it did — granted
/// unconditionally, same judgment call as other rounds' no-cost-field cases (e.g. Ib's
/// unconditional grants). `shop_id` is treated as the motion id to unlock directly.
pub async fn handle(pool: &SqlitePool, uid: i64, req: AvatarMotionShopBuyRequest) -> GameResponse {
    info!("Handling AvatarMotionShopBuyRequest: {:?}", req);

    let reward_info_bundle = if let Some(shop_id) = req.shop_id {
        let _ = avatar_motion_info::grant(pool, uid, shop_id).await;
        Some(RewardDbInfoBundle {
            item_info: vec![ItemDbInfo {
                id: Some(shop_id),
                r#type: Some(AVATAR_MOTION_ITEM_TYPE_PLACEHOLDER),
                count: Some(1),
                ..Default::default()
            }],
            ..Default::default()
        })
    } else {
        None
    };

    let response = AvatarMotionShopBuyResponse { reward_info_bundle };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::AvatarMotionShopBuy.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
