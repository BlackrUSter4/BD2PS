pub mod avatar_info;
pub mod avatar_motion_shop_buy;
pub mod avatar_save;
pub mod avatar_shop_buy;
pub mod avatar_shop_wish_list_info;
pub mod avatar_shop_wish_list_save;

use bd2::proto::proto_net::ItemDbInfo;
use sqlx::SqlitePool;

/// This client build has no captured `AvatarItemTable`/`AvatarShopTable`/`AvatarMotionShopTable`
/// data at all (a genuinely new system with zero rows for every backing table) — there is no
/// real item-category value to attach to a shop-bought avatar item, and no real "this is an
/// avatar motion" item type to tag a granted motion with in a generic `ItemDbInfo`. These are
/// placeholders until real master data gets captured for this feature.
pub const AVATAR_ITEM_CATEGORY_PLACEHOLDER: i32 = 0;
pub const AVATAR_MOTION_ITEM_TYPE_PLACEHOLDER: i32 = 9002;

/// Consume a claimed cost from the account's general item inventory. Same pattern as
/// fishing/life/my_room — the client always specifies exactly what it's spending, so no
/// separate cost table is required to validate against.
pub async fn try_consume_items(pool: &SqlitePool, uid: i64, items: &[ItemDbInfo]) -> bool {
    for item in items {
        let Some(id) = item.id else { continue };
        let count = item.count.unwrap_or(1);
        match database::db::item::item_info::consume(pool, uid, id, count).await {
            Ok(true) => {}
            _ => return false,
        }
    }
    true
}
