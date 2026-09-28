use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeShopDbInfo, EvilCastleRogueLikeShopItemInfo as ShopItemMsg, EvilCastleRogueLikeShopReRollRequest, EvilCastleRogueLikeShopReRollResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::evil::{evil_castle_rogue_like_info, evil_castle_rogue_like_shop_info, evil_castle_rogue_like_shop_item_info};
use database::models::game::evil::evil_castle_rogue_like_shop_item_info::EvilCastleRogueLikeShopItemInfo;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeShopReRollRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeShopReRollRequest: {:?}", req);
    let _ = req;

    let shop_meta = evil_castle_rogue_like_shop_info::get_one(pool, uid).await.ok().flatten();
    let price = shop_meta.as_ref().and_then(|s| s.re_roll_price).unwrap_or(0);
    let mut used_gold = 0;
    let mut shop_info = None;

    if let Some(mut run) = evil_castle_rogue_like_info::get_one(pool, uid).await.ok().flatten() {
        if run.rogue_like_gold.unwrap_or(0) >= price {
            run.rogue_like_gold = Some(run.rogue_like_gold.unwrap_or(0) - price);
            let _ = evil_castle_rogue_like_info::upsert(pool, &run).await;
            used_gold = price;

            let relics = &exceldb::get().rlrelictable;
            let mut seed = super::roguelike::new_seed(uid + chrono::Utc::now().timestamp_millis());
            let mut items = vec![];
            for _ in 0..4 {
                if let Some(r) = relics.all().get((super::roguelike::rand_u32(&mut seed) as usize) % relics.all().len().max(1)) {
                    items.push(EvilCastleRogueLikeShopItemInfo { index: 0, uid, r#type: Some(1), id: Some(r.id), price: r.relic_price, sold_out: Some(0) });
                }
            }
            let _ = evil_castle_rogue_like_shop_item_info::save_all(pool, uid, &items).await;
            shop_info = Some(EvilCastleRogueLikeShopDbInfo {
                item_info: items.into_iter().map(|i| ShopItemMsg { r#type: i.r#type, id: i.id, price: i.price, sold_out: i.sold_out }).collect(),
                re_roll_price: Some(price),
            });
        }
    }

    let response = EvilCastleRogueLikeShopReRollResponse { shop_info, used_gold: Some(used_gold) };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeShopReRoll.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
