use bd2::prost::Message;
use bd2::proto::proto_net::{
    CafeteriaRareNpcInteractionRewardRequest, CafeteriaRareNpcInteractionRewardResponse,
    ItemDbInfo, RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{cafeteria::cafeteria_info, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

/// `CafeteriaUniqueNpcSpawnTable` (rare NPCs) only carries spawn-timing config, no reward
/// fields at all — flat placeholder gold. The daily currency-count CAP is real
/// (`CafeteriaDefaultTable.dailyShopCurrencyLimit`).
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: CafeteriaRareNpcInteractionRewardRequest,
) -> GameResponse {
    info!("Handling CafeteriaRareNpcInteractionRewardRequest: {:?}", req);

    let mut info_row = match cafeteria_info::get_or_create(pool, uid).await {
        Ok(row) => row,
        Err(e) => {
            tracing::error!("CafeteriaRareNpcInteractionReward get_or_create failed: {}", e);
            return GameResponse::error(1);
        }
    };

    let cap = data::exceldb::get()
        .cafeteriadefaulttable
        .all()
        .first()
        .map(|d| d.daily_shop_currency_limit)
        .unwrap_or(20);
    let current = info_row.daily_npc_reward_currency_count.unwrap_or(0);

    let mut item_infos = Vec::new();
    if current < cap {
        let _ = item_info::grant(
            pool,
            uid,
            super::GOLD_ITEM_ID,
            super::GOLD_ITEM_TYPE,
            super::RARE_NPC_REWARD_GOLD,
        )
        .await;
        item_infos.push(ItemDbInfo {
            id: Some(super::GOLD_ITEM_ID),
            r#type: Some(super::GOLD_ITEM_TYPE),
            count: Some(super::RARE_NPC_REWARD_GOLD),
            ..Default::default()
        });
        info_row.daily_npc_reward_currency_count = Some(current + 1);
        if let Err(e) = cafeteria_info::update_cafeteria_info(pool, &info_row).await {
            tracing::error!("CafeteriaRareNpcInteractionReward update failed: {}", e);
        }
    }

    let response = CafeteriaRareNpcInteractionRewardResponse {
        reward_info: Some(RewardDbInfoBundle {
            item_info: item_infos,
            ..Default::default()
        }),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CafeteriaRareNpcInteractionReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
