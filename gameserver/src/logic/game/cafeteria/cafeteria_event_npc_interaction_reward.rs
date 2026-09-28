use bd2::prost::Message;
use bd2::proto::proto_net::{
    CafeteriaEventNpcInteractionRewardRequest, CafeteriaEventNpcInteractionRewardResponse,
    ItemDbInfo, RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{cafeteria::cafeteria_info, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

/// Real reward: `CafeteriaEventTable` (keyed by groupId+id — its own `id` field is not globally
/// unique across groups) has real `rewardType`/`rewardCount` per event. Granted as gold, since
/// no table names an actual item id for that reward type (same recurring gap seen everywhere
/// else in Cafeteria). The daily currency-count cap is real.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: CafeteriaEventNpcInteractionRewardRequest,
) -> GameResponse {
    info!("Handling CafeteriaEventNpcInteractionRewardRequest: {:?}", req);

    let Some(group_id) = req.event_group_id else {
        return GameResponse::error(1);
    };
    let Some(event_id) = req.event_id else {
        return GameResponse::error(1);
    };

    let mut info_row = match cafeteria_info::get_or_create(pool, uid).await {
        Ok(row) => row,
        Err(e) => {
            tracing::error!("CafeteriaEventNpcInteractionReward get_or_create failed: {}", e);
            return GameResponse::error(1);
        }
    };

    let event = data::exceldb::get()
        .cafeteriaeventtable
        .by_group(group_id)
        .find(|e| e.id == event_id)
        .map(|e| (e.reward_type, e.reward_count));

    let cap = data::exceldb::get()
        .cafeteriadefaulttable
        .all()
        .first()
        .map(|d| d.daily_shop_currency_limit)
        .unwrap_or(20);
    let current = info_row.daily_npc_reward_currency_count.unwrap_or(0);

    let mut item_infos = Vec::new();
    if let Some((reward_type, reward_count)) = event {
        if current < cap {
            let _ = item_info::grant(pool, uid, super::GOLD_ITEM_ID, reward_type, reward_count).await;
            item_infos.push(ItemDbInfo {
                id: Some(super::GOLD_ITEM_ID),
                r#type: Some(reward_type),
                count: Some(reward_count),
                ..Default::default()
            });
            info_row.daily_npc_reward_currency_count = Some(current + 1);
            if let Err(e) = cafeteria_info::update_cafeteria_info(pool, &info_row).await {
                tracing::error!("CafeteriaEventNpcInteractionReward update failed: {}", e);
            }
        }
    }

    let response = CafeteriaEventNpcInteractionRewardResponse {
        reward_info: Some(RewardDbInfoBundle {
            item_info: item_infos,
            ..Default::default()
        }),
        daily_npc_reward_currency_count: info_row.daily_npc_reward_currency_count,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CafeteriaEventNpcInteractionReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
