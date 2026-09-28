use bd2::prost::Message;
use bd2::proto::proto_net::{IdCardShopBuyRequest, IdCardShopBuyResponse, ItemDbInfo, RewardDbInfoBundle, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{id::id_card_shop_info as shop_db, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

/// Real purchase: validates against IdCardItemTshopTable's real buy_max_count cap and
/// real price_id/type/count, consuming the real price (not just whatever the client's
/// `use_item_info` claims) before granting the id-card item.
pub async fn handle(pool: &SqlitePool, uid: i64, req: IdCardShopBuyRequest) -> GameResponse {
    info!("Handling IdCardShopBuyRequest: {:?}", req);

    let mut item_infos = Vec::new();
    for entry in &req.item_info {
        let Some(id) = entry.id else { continue };
        let Some(shop_item) = data::exceldb::get().idcarditemtshoptable.get(id) else { continue };

        let already_bought = shop_db::get_by_uid_and_id(pool, uid, id).await.ok().flatten()
            .and_then(|r| r.buy_count).unwrap_or(0);
        let requested = entry.item_count.unwrap_or(1).max(1);
        if already_bought + requested > shop_item.buy_max_count {
            continue;
        }

        let mut can_afford = true;
        for i in 0..shop_item.price_id.len() {
            let cost = shop_item.price_count.get(i).copied().unwrap_or(0) * requested;
            if cost > 0 {
                let has = item_info::consume(pool, uid, shop_item.price_id[i], cost).await.unwrap_or(false);
                if !has {
                    can_afford = false;
                    break;
                }
            }
        }
        if !can_afford {
            continue;
        }

        let _ = shop_db::increment_buy_count(pool, uid, id, requested).await;
        let _ = item_info::grant(pool, uid, id, 1, requested).await;
        item_infos.push(ItemDbInfo { id: Some(id), r#type: Some(1), count: Some(requested), ..Default::default() });
    }

    let response = IdCardShopBuyResponse {
        reward_info_bundle: Some(RewardDbInfoBundle { item_info: item_infos, ..Default::default() }),
    };
    
    let resp_bytes = response.encode_to_vec();
    
    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };
    
    let (route, code) = PacketCodeType::IdCardShopBuy.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}