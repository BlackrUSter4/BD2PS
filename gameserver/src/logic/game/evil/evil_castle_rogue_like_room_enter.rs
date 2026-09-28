use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeEventInfo as EventInfoMsg, EvilCastleRogueLikeRoomEnterRequest, EvilCastleRogueLikeRoomEnterResponse, EvilCastleRogueLikeShopDbInfo, EvilCastleRogueLikeShopItemInfo as ShopItemMsg, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::evil::{
    evil_castle_rogue_like_event_info, evil_castle_rogue_like_room_info,
    evil_castle_rogue_like_shop_info, evil_castle_rogue_like_shop_item_info, evil_castle_rogue_like_state_info,
};
use database::models::game::evil::evil_castle_rogue_like_event_info::EvilCastleRogueLikeEventInfo;
use database::models::game::evil::evil_castle_rogue_like_shop_info::EvilCastleRogueLikeShopInfo;
use database::models::game::evil::evil_castle_rogue_like_shop_item_info::EvilCastleRogueLikeShopItemInfo;
use database::models::game::evil::evil_castle_rogue_like_state_info::EvilCastleRogueLikeStateInfo;
use sqlx::SqlitePool;
use tracing::info;

use super::roguelike;

/// Entering a room resolves what's there — event, shop, or just a battle
/// (state_info.state advances; battle rooms have nothing extra to roll).
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeRoomEnterRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeRoomEnterRequest: {:?}", req);

    let room_number = req.room_number.unwrap_or(0);
    let state = evil_castle_rogue_like_state_info::get_one(pool, uid).await.ok().flatten();
    let floor = state.as_ref().and_then(|s| s.floor).unwrap_or(1);
    let rooms = evil_castle_rogue_like_room_info::get_by_floor(pool, uid, floor).await.unwrap_or_default();
    let room = rooms.into_iter().find(|r| r.number == Some(room_number));

    let mut event_info = None;
    let mut shop_info = None;
    let db = exceldb::get();

    if let Some(r) = &room {
        if let Some(room_id) = r.id {
            if let Some(room_def) = db.rlroomtable.get(room_id) {
                if room_def.r#type == 2 {
                    // event room
                    if let Some(choice_def) = db.rleventchoicetable.get(room_def.event_choice_id) {
                        if let Some(&group_id) = choice_def.event_group_id.first() {
                            if let Some(ev) = db.rleventtable.by_group(group_id).next() {
                                let ev_info = EvilCastleRogueLikeEventInfo { index: 0, uid, group_id: Some(group_id), id: Some(ev.id) };
                                let _ = evil_castle_rogue_like_event_info::upsert(pool, &ev_info).await;
                                event_info = Some(EventInfoMsg { group_id: Some(group_id), id: Some(ev.id) });
                            }
                        }
                    }
                } else if room_def.r#type == 3 {
                    // shop room
                    if let Some(shop_def) = db.rlshoptable.get(room_def.shop_table_id) {
                        let mut seed = roguelike::new_seed(uid + room_number as i64);
                        let mut items = vec![];
                        for _ in 0..4 {
                            let idx = (roguelike::rand_u32(&mut seed) as usize) % db.rlrelictable.all().len().max(1);
                            if let Some(relic) = db.rlrelictable.all().get(idx) {
                                items.push(EvilCastleRogueLikeShopItemInfo {
                                    index: 0,
                                    uid,
                                    r#type: Some(1),
                                    id: Some(relic.id),
                                    price: relic.relic_price,
                                    sold_out: Some(0),
                                });
                            }
                        }
                        let _ = evil_castle_rogue_like_shop_item_info::save_all(pool, uid, &items).await;
                        let shop_meta = EvilCastleRogueLikeShopInfo { index: 0, uid, item_info_index: None, re_roll_price: Some(shop_def.reroll_price) };
                        let _ = evil_castle_rogue_like_shop_info::upsert(pool, &shop_meta).await;
                        shop_info = Some(EvilCastleRogueLikeShopDbInfo {
                            item_info: items.into_iter().map(|i| ShopItemMsg { r#type: i.r#type, id: i.id, price: i.price, sold_out: i.sold_out }).collect(),
                            re_roll_price: Some(shop_def.reroll_price),
                        });
                    }
                }
            }
        }
    }

    let new_state = EvilCastleRogueLikeStateInfo { index: 0, uid, floor: Some(floor), room: Some(room_number), state: Some(1) };
    let _ = evil_castle_rogue_like_state_info::upsert(pool, &new_state).await;

    let response = EvilCastleRogueLikeRoomEnterResponse {
        state_info: Some(roguelike::default_state(floor, room_number)),
        choice_info: None,
        event_info,
        shop_info,
        battle_level: None,
        rogue_like_gold: None,
        clear_floor: None,
        clear_room_info: None,
    };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeRoomEnter.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
