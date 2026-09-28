use bd2::prost::Message;
use bd2::proto::proto_net::{FriendshipDbInfo, FriendshipGiftRequest, FriendshipGiftResponse, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::friendship::friendship_info;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

use super::GIFT_EXP_PER_ITEM;

/// No `FriendshipCostumeTable`/gift-value data links a gifted item to an exp amount — real item
/// consumption (only items the account actually has, in the quantities sent), but a flat
/// placeholder exp-per-consumed-unit. The reward bundle is always empty: gifting isn't described
/// anywhere as granting a separate reward on top of exp.
pub async fn handle(pool: &SqlitePool, uid: i64, req: FriendshipGiftRequest) -> GameResponse {
    info!("Handling FriendshipGiftRequest: {:?}", req);

    let costume_id = req.costume_id.unwrap_or_default();

    let mut consumed_units = 0i32;
    for item in &req.item_info {
        let Some(id) = item.id else { continue };
        let count = item.count.unwrap_or(1).max(1);
        if item_info::consume(pool, uid, id, count).await.unwrap_or(false) {
            consumed_units += count;
        }
    }

    let gained_exp = consumed_units.saturating_mul(GIFT_EXP_PER_ITEM);
    let info_row = friendship_info::add_exp(pool, uid, costume_id, gained_exp, false).await;
    let friendship_info_out = FriendshipDbInfo {
        friendship_costume_id: Some(info_row.costume_id),
        level: Some(info_row.level),
        exp: Some(info_row.exp),
        last_counseling_date: info_row.last_counseling_date,
    };

    let response = FriendshipGiftResponse {
        reward_info_bundle: Some(RewardDbInfoBundle::default()),
        friendship_info: Some(friendship_info_out),
        gained_exp: Some(gained_exp),
    };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::FriendshipGift.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
