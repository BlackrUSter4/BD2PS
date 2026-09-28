use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeShopBuyRequest, EvilCastleRogueLikeShopBuyResponse, Notify, RelicDbInfo};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::{evil_castle_rogue_like_info, evil_castle_rogue_like_shop_item_info};
use database::db::relic::relic_info;
use database::models::game::relic::relic_info::RelicInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Buys one item from the current room's shop (type 1 = relic, the only
/// kind this shop stocks currently) — real gold spend + real ownership.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeShopBuyRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeShopBuyRequest: {:?}", req);

    let mut relic_out = None;
    let mut used_gold = 0;

    if let (Some(id), Some(mut run)) = (req.id, evil_castle_rogue_like_info::get_one(pool, uid).await.ok().flatten()) {
        let items = evil_castle_rogue_like_shop_item_info::get_evil_castle_rogue_like_shop_item_info(pool, uid).await.unwrap_or_default();
        if let Some(item) = items.iter().find(|i| i.id == Some(id) && i.sold_out != Some(1)) {
            let price = item.price.unwrap_or(0);
            if run.rogue_like_gold.unwrap_or(0) >= price {
                run.rogue_like_gold = Some(run.rogue_like_gold.unwrap_or(0) - price);
                let _ = evil_castle_rogue_like_info::upsert(pool, &run).await;
                used_gold = price;

                let new_index = chrono::Utc::now().timestamp_millis();
                let relic = RelicInfo { index: 0, uid, inven_index: Some(new_index), id: Some(id) };
                let _ = relic_info::insert(pool, &relic).await;
                relic_out = Some(RelicDbInfo { inven_index: Some(new_index), id: Some(id) });

                let mut updated = items;
                for i in updated.iter_mut() {
                    if i.id == Some(id) {
                        i.sold_out = Some(1);
                    }
                }
                let _ = evil_castle_rogue_like_shop_item_info::save_all(pool, uid, &updated).await;
            }
        }
    }

    let response = EvilCastleRogueLikeShopBuyResponse { char_info: vec![], costume_info: None, relic_info: relic_out, used_gold: Some(used_gold) };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeShopBuy.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
