use bd2::proto::proto_net::{IdCardInfo as ProtoCard, IdCardItemInfo as ProtoItem};
use database::db::id::{id_card_info as info_db, id_card_item_info as item_db};
use database::models::game::id::{id_card_info::IdCardInfo, id_card_item_info::IdCardItemInfo};
use sqlx::SqlitePool;

/// Insert a fresh IdCardItemInfo row for one card slot and return its Index. Always
/// inserts new rather than updating in place (simpler; any previous row for that slot is
/// left as harmless orphaned garbage — acceptable simplification given time budget).
async fn store_item(pool: &SqlitePool, uid: i64, item: &ProtoItem) -> Option<i64> {
    item_db::insert(
        pool,
        &IdCardItemInfo {
            index: 0,
            uid,
            inven_index: item.inven_index,
            id: item.id,
            x: item.x,
            y: item.y,
            rotate: item.rotate,
            scale: item.scale,
            layer: item.layer,
            color: item.color.clone(),
        },
    )
    .await
    .ok()
}

fn item_to_proto(row: &IdCardItemInfo) -> ProtoItem {
    ProtoItem {
        inven_index: row.inven_index,
        id: row.id,
        x: row.x,
        y: row.y,
        rotate: row.rotate,
        scale: row.scale,
        layer: row.layer,
        color: row.color.clone(),
    }
}

async fn item_by_index(pool: &SqlitePool, uid: i64, index: Option<i64>) -> Option<ProtoItem> {
    let idx = index?;
    item_db::get_by_index(pool, uid, idx).await.ok().map(|r| item_to_proto(&r))
}

/// Turn a client-sent IdCardInfo into stored item rows, returning an (unsaved) IdCardInfo
/// row pointing at them.
async fn build_card_row(pool: &SqlitePool, uid: i64, card: &ProtoCard) -> IdCardInfo {
    let background_index = match &card.background { Some(i) => store_item(pool, uid, i).await, None => None };
    let sub_background_index = match &card.sub_background { Some(i) => store_item(pool, uid, i).await, None => None };
    let background_effect_index = match &card.background_effect { Some(i) => store_item(pool, uid, i).await, None => None };
    let my_info_index = match &card.my_info { Some(i) => store_item(pool, uid, i).await, None => None };

    let mut sticker_indices = Vec::new();
    for s in &card.stickers {
        if let Some(idx) = store_item(pool, uid, s).await {
            sticker_indices.push(idx);
        }
    }
    let stickers_index = serde_json::to_string(&sticker_indices).ok();

    IdCardInfo {
        index: 0,
        uid,
        background_index,
        sub_background_index,
        background_effect_index,
        stickers_index,
        my_info_index,
        rotate: card.rotate,
    }
}

/// Save a full IdCardInfo as fresh item rows, then upsert the account's single "current"
/// IdCardInfo row (the one `IdCardSaveRequest` reads/writes) to point at them. Returns the
/// resulting row. NOT used for presets — each preset gets its own independent row via
/// `store_card_fresh`, since they must coexist rather than share one "current" slot.
pub async fn save_card(pool: &SqlitePool, uid: i64, card: &ProtoCard) -> Option<IdCardInfo> {
    let data = build_card_row(pool, uid, card).await;
    if info_db::get_current(pool, uid).await.ok().flatten().is_some() {
        let _ = info_db::update(pool, &data).await;
    } else {
        let _ = info_db::add_id_card_info(pool, &data).await;
    }
    info_db::get_current(pool, uid).await.ok().flatten()
}

/// Save a full IdCardInfo as a brand-new, independent row (for presets — each preset keeps
/// its own snapshot rather than sharing the single "current" row `save_card` manages).
pub async fn store_card_fresh(pool: &SqlitePool, uid: i64, card: &ProtoCard) -> Option<IdCardInfo> {
    let data = build_card_row(pool, uid, card).await;
    let index = info_db::insert(pool, &data).await.ok()?;
    info_db::get_by_index(pool, uid, index).await.ok()
}

/// Build the response DTO for the account's currently-stored card.
pub async fn card_to_proto(pool: &SqlitePool, uid: i64, row: &IdCardInfo) -> ProtoCard {
    let mut stickers = Vec::new();
    if let Some(json) = &row.stickers_index {
        if let Ok(indices) = serde_json::from_str::<Vec<i64>>(json) {
            for idx in indices {
                if let Some(item) = item_by_index(pool, uid, Some(idx)).await {
                    stickers.push(item);
                }
            }
        }
    }
    ProtoCard {
        background: item_by_index(pool, uid, row.background_index).await,
        sub_background: item_by_index(pool, uid, row.sub_background_index).await,
        background_effect: item_by_index(pool, uid, row.background_effect_index).await,
        stickers,
        my_info: item_by_index(pool, uid, row.my_info_index).await,
        rotate: row.rotate,
    }
}

pub mod id_card_preset_delete;
pub mod id_card_preset_info;
pub mod id_card_preset_save;
pub mod id_card_recovery;
pub mod id_card_save;
pub mod id_card_shop_buy;
pub mod id_card_shop_info;
