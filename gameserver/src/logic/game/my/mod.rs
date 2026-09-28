pub mod my_like_info;
pub mod my_room_expand;
pub mod my_room_hide;
pub mod my_room_item_info;
pub mod my_room_like;
pub mod my_room_move;
pub mod my_room_name_change;
pub mod my_room_preset_delete;
pub mod my_room_preset_info;
pub mod my_room_preset_name_change;
pub mod my_room_preset_save;
pub mod my_room_primary_change;
pub mod my_room_save;
pub mod my_room_search_friend;
pub mod my_room_search_guild;
pub mod my_room_search_recommend;
pub mod my_room_share_option_update;
pub mod my_room_shop_buy;
pub mod my_room_shop_info;
pub mod my_room_shop_multi_buy;
pub mod my_room_user_info;

use bd2::proto::proto_net::{
    ItemDbInfo, MyRoomDbInfo, MyRoomItemPositionInfo as MyRoomItemPositionInfoProto,
    MyRoomTrophyDbInfo, MyRoomUserInfo as MyRoomUserInfoProto,
};
use database::db::my::{
    my_room_info, my_room_item_position_info, my_room_trophy_info,
    my_room_user_info as db_my_room_user_info,
};
use sqlx::SqlitePool;

pub const DEFAULT_ROOM_ID: i32 = 1;

/// Consume a claimed cost from the account's general item inventory. Mirrors the same
/// pattern used by fishing/life/colosseum — the client always specifies exactly what it's
/// spending, so no separate cost table is required to validate against.
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

/// Make sure a brand-new account has a first room and a MyRoomUserInfo profile row to
/// attach likes/primary-room/share-option state to. No-ops if either already exists.
pub async fn ensure_default_room(pool: &SqlitePool, uid: i64) {
    if let Ok(rooms) = my_room_info::get_my_room_info(pool, uid).await {
        if rooms.is_empty() {
            let _ = my_room_info::insert(
                pool,
                &database::models::game::my::my_room_info::MyRoomInfo {
                    index: 0,
                    uid,
                    id: Some(DEFAULT_ROOM_ID),
                    name: Some("My Room".to_string()),
                    is_hidden: Some(0),
                },
            )
            .await;
        }
    }

    if let Ok(None) = db_my_room_user_info::get_one(pool, uid).await {
        let _ = db_my_room_user_info::insert(
            pool,
            &database::models::game::my::my_room_user_info::MyRoomUserInfo {
                index: 0,
                uid,
                owner_index: Some(uid),
                user_id: None,
                portrait_costume_id: None,
                primary_my_room_id: Some(DEFAULT_ROOM_ID),
                item_info_index: None,
                costume_info_index: None,
                trophy_info_index: None,
                my_room_index: None,
                my_room_like_count: Some(0),
                my_room_like_date: Some(0),
                portrait_costume_design_id: None,
                allow_scope_type: Some(0),
            },
        )
        .await;
    }
}

fn position_row_to_proto(row: &database::models::game::my::my_room_item_position_info::MyRoomItemPositionInfo) -> MyRoomItemPositionInfoProto {
    MyRoomItemPositionInfoProto {
        inven_index: row.inven_index,
        object_type: row.object_type,
        position_type: row.position_type,
        x: row.x,
        y: row.y,
        rotate: row.rotate,
        interact: row.interact,
        item_animation: row.item_animation,
        is_wall_hidden: row.is_wall_hidden,
    }
}

/// Build the full list of a room-owning account's rooms (each with its placed items).
pub async fn build_my_room_list(pool: &SqlitePool, uid: i64) -> Vec<MyRoomDbInfo> {
    let Ok(rooms) = my_room_info::get_my_room_info(pool, uid).await else {
        return vec![];
    };
    let mut out = Vec::with_capacity(rooms.len());
    for room in rooms {
        let room_id = room.id.unwrap_or(DEFAULT_ROOM_ID);
        let positions = my_room_item_position_info::get_by_uid_and_room(pool, uid, room_id)
            .await
            .unwrap_or_default();
        out.push(MyRoomDbInfo {
            id: room.id,
            name: room.name,
            my_room_position_info: positions.iter().map(position_row_to_proto).collect(),
            is_hidden: room.is_hidden.map(|v| v != 0),
        });
    }
    out
}

pub async fn build_trophy_list(pool: &SqlitePool, uid: i64) -> Vec<MyRoomTrophyDbInfo> {
    my_room_trophy_info::get_my_room_trophy_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|row| MyRoomTrophyDbInfo {
            inven_index: row.inven_index,
            id: row.id,
            contents_type: row.contents_type,
            season: row.season,
            use_count: row.use_count,
        })
        .collect()
}

/// Build a full MyRoomUserInfo profile snapshot for `uid` (works for the caller's own
/// account or any other real account — used by UserInfo/Like/Search*).
pub async fn build_user_info(pool: &SqlitePool, uid: i64) -> MyRoomUserInfoProto {
    let profile = db_my_room_user_info::get_one(pool, uid).await.ok().flatten();
    let account = database::db::user::user::find_account(pool, uid)
        .await
        .ok()
        .flatten();
    let items = database::db::item::item_info::get_item_info(pool, uid)
        .await
        .unwrap_or_default();

    MyRoomUserInfoProto {
        owner_index: Some(uid),
        user_id: account.map(|a| a.user_name),
        portrait_costume_id: profile.as_ref().and_then(|p| p.portrait_costume_id),
        primary_my_room_id: profile
            .as_ref()
            .and_then(|p| p.primary_my_room_id)
            .or(Some(DEFAULT_ROOM_ID)),
        item_info: items
            .into_iter()
            .map(|i| ItemDbInfo {
                inven_index: i.inven_index,
                id: i.id,
                count: i.count,
                r#type: i.r#type,
                ..Default::default()
            })
            .collect(),
        costume_info: vec![],
        trophy_info: build_trophy_list(pool, uid).await,
        my_room: build_my_room_list(pool, uid).await,
        my_room_like_count: profile.as_ref().and_then(|p| p.my_room_like_count),
        my_room_like_date: profile.as_ref().and_then(|p| p.my_room_like_date),
        portrait_costume_design_id: profile.as_ref().and_then(|p| p.portrait_costume_design_id),
    }
}
