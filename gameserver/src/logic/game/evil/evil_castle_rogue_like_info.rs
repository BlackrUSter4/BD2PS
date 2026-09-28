use bd2::prost::Message;
use bd2::proto::proto_net::{
    CharDbInfo, CostumeDbInfo, DeckDbInfo, EvilCastleRogueLikeChoiceInfo as ChoiceInfoMsg,
    EvilCastleRogueLikeEventInfo as EventInfoMsg, EvilCastleRogueLikeGrowthDbInfo,
    EvilCastleRogueLikeInfoRequest, EvilCastleRogueLikeInfoResponse,
    EvilCastleRogueLikeShopDbInfo, EvilCastleRogueLikeShopItemInfo as ShopItemMsg, Notify, RelicDbInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::char::char_info;
use database::db::costume::costume_info;
use database::db::evil::{
    evil_castle_rogue_like_choice_info, evil_castle_rogue_like_deck_info, evil_castle_rogue_like_event_info,
    evil_castle_rogue_like_growth_info, evil_castle_rogue_like_info, evil_castle_rogue_like_shop_info,
    evil_castle_rogue_like_shop_item_info, evil_castle_rogue_like_state_info,
};
use database::db::relic::relic_info;
use sqlx::SqlitePool;
use tracing::info;

use super::roguelike;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeInfoRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeInfoRequest: {:?}", req);

    let run = evil_castle_rogue_like_info::get_one(pool, uid).await.ok().flatten();
    let state = evil_castle_rogue_like_state_info::get_one(pool, uid).await.ok().flatten();
    let current_floor = state.as_ref().and_then(|s| s.floor).unwrap_or(1);

    let floor_info = roguelike::get_all_floors(pool, uid, current_floor).await;

    let deck_info = evil_castle_rogue_like_deck_info::get(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|d| DeckDbInfo {
            char_inven_index: Some(d.char_inven_index),
            position: Some(d.position),
            sequence: Some(d.sequence),
        })
        .collect();

    let char_info: Vec<CharDbInfo> = char_info::get_char_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|c| CharDbInfo {
            inven_index: c.inven_index,
            id: c.id,
            hp: c.hp,
            level: c.level,
            costume_id: c.costume_id,
            exp: c.exp,
            use_costume: c.use_costume,
            talent_level: c.talent_level,
            talent_exp: c.talent_exp,
            solidarity_reward: c.solidarity_reward,
            expiry_time: c.expiry_time,
            pictorialbook_info: vec![],
            connect_potential_costume: c.connect_potential_costume,
        })
        .collect();

    let costume_info: Vec<CostumeDbInfo> = costume_info::get_costume_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|c| CostumeDbInfo {
            inven_index: c.inven_index,
            id: c.id,
            design_id: c.design_id,
            level: c.level,
            ..Default::default()
        })
        .collect();

    let relic_info: Vec<RelicDbInfo> = relic_info::get_relic_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| RelicDbInfo {
            inven_index: r.inven_index,
            id: r.id,
        })
        .collect();

    let choice_info = evil_castle_rogue_like_choice_info::get_evil_castle_rogue_like_choice_info(pool, uid)
        .await
        .ok()
        .and_then(|v| v.into_iter().next())
        .map(|c| ChoiceInfoMsg {
            r#type: c.r#type,
            id: c.ids.and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default(),
        });

    let growth_info: Vec<EvilCastleRogueLikeGrowthDbInfo> = evil_castle_rogue_like_growth_info::get_evil_castle_rogue_like_growth_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|g| EvilCastleRogueLikeGrowthDbInfo {
            r#type: g.r#type,
            level: g.level,
        })
        .collect();

    let event_info = evil_castle_rogue_like_event_info::get_one(pool, uid)
        .await
        .ok()
        .flatten()
        .map(|e| EventInfoMsg {
            group_id: e.group_id,
            id: e.id,
        });

    let shop_meta = evil_castle_rogue_like_shop_info::get_one(pool, uid).await.ok().flatten();
    let shop_items = evil_castle_rogue_like_shop_item_info::get_evil_castle_rogue_like_shop_item_info(pool, uid)
        .await
        .unwrap_or_default();
    let shop_info = shop_meta.map(|s| EvilCastleRogueLikeShopDbInfo {
        item_info: shop_items
            .into_iter()
            .map(|i| ShopItemMsg {
                r#type: i.r#type,
                id: i.id,
                price: i.price,
                sold_out: i.sold_out,
            })
            .collect(),
        re_roll_price: s.re_roll_price,
    });

    let response = EvilCastleRogueLikeInfoResponse {
        state_info: state.map(|s| roguelike::default_state(s.floor.unwrap_or(1), s.room.unwrap_or(0))),
        level: run.as_ref().and_then(|r| r.level),
        floor_info,
        deck_info,
        char_info,
        costume_info,
        relic_info,
        choice_info,
        re_roll: run.as_ref().and_then(|r| r.re_roll),
        group_id: run.as_ref().and_then(|r| r.group_id),
        id: run.as_ref().and_then(|r| r.id),
        growth_info,
        event_info,
        shop_info,
        battle_level: run.as_ref().and_then(|r| r.battle_level),
        max_try_level: run.as_ref().and_then(|r| r.max_try_level),
        obsidian: run.as_ref().and_then(|r| r.obsidian),
        rogue_like_gold: run.as_ref().and_then(|r| r.rogue_like_gold),
        season: run.as_ref().and_then(|r| r.season),
        regular_season: run.as_ref().and_then(|r| r.regular_season),
        season_reward: run.as_ref().and_then(|r| r.season_reward).map(|v| v != 0),
        max_reward_level: run.as_ref().and_then(|r| r.max_reward_level),
        highest_crystal_damage: run.as_ref().and_then(|r| r.highest_crystal_damage),
    };

    let resp_bytes = response.encode_to_vec();
    let notify = Notify {
        active_login_event: vec![1, 2],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        ..Default::default()
    };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
