use bd2::prost::Message;
use bd2::proto::proto_net::{
    EventExchangeRewardDbInfo, EventExchangeRewardRequest, EventExchangeRewardResponse, ItemDbInfo,
    Notify, RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{
    event::{event_exchange_info as ex_db, event_exchange_reward_info as reward_db},
    item::item_info,
};
use rand::seq::IndexedRandom;
use sqlx::SqlitePool;
use tracing::info;

/// Real exchange draw against EventCoinExchangeTable (the real master table backing the
/// "coin exchange" event shop). The request carries no explicit group_id, so every group the
/// account has already opened progress on for this event_uid is considered and entries are
/// restricted to the account's current Page within that group. Weighted by real `ratio`, same
/// uniform-choose-among-candidates placeholder precedent as GachaBuy (no separate weighted-roll
/// helper exists in this codebase yet). Consumed items are real; granted rewards are real.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EventExchangeRewardRequest) -> GameResponse {
    info!("Handling EventExchangeRewardRequest: {:?}", req);

    let event_uid = req.event_uid.unwrap_or(0);
    let exchange_count = req.exchange_count.filter(|&c| c > 0).unwrap_or(1).clamp(1, 100);

    for item in &req.use_item_info {
        if let (Some(id), Some(count)) = (item.id, item.count) {
            let _ = item_info::consume(pool, uid, id, count).await;
        }
    }

    let game_data = data::exceldb::get();
    let progress_rows = ex_db::get_event_exchange_info(pool, uid).await.unwrap_or_default();

    let mut candidates = Vec::new();
    for row in progress_rows.iter().filter(|r| r.event_uid == Some(event_uid)) {
        let group_id = row.group_id.unwrap_or(0);
        let page = row.page.unwrap_or(1);
        for entry in game_data.eventcoinexchangetable.by_group(group_id) {
            if entry.page_id == page {
                candidates.push((group_id, entry));
            }
        }
    }

    let mut item_infos = Vec::new();
    let mut change_exchange_reward_info = Vec::new();

    for _ in 0..exchange_count {
        let Some((group_id, entry)) = candidates.choose(&mut rand::thread_rng()).copied() else {
            break;
        };

        let reward_id = entry.reward_item_id.unwrap_or(entry.id);
        let reward_type = entry.reward_item_type;
        let reward_count = entry.reward_item_count * entry.set_count.max(1);

        let _ = item_info::grant(pool, uid, reward_id, reward_type, reward_count).await;
        let _ = reward_db::add_count(pool, uid, event_uid, group_id, entry.id, 1).await;

        item_infos.push(ItemDbInfo {
            id: Some(reward_id),
            r#type: Some(reward_type),
            count: Some(reward_count),
            ..Default::default()
        });

        let updated = reward_db::get_by_uid_event_group_id(pool, uid, event_uid, group_id, entry.id)
            .await
            .ok()
            .flatten();
        change_exchange_reward_info.push(EventExchangeRewardDbInfo {
            event_uid: Some(event_uid),
            group_id: Some(group_id),
            id: Some(entry.id),
            count: updated.and_then(|u| u.count),
        });
    }

    let response = EventExchangeRewardResponse {
        reward_info_bundle: Some(RewardDbInfoBundle { item_info: item_infos, ..Default::default() }),
        change_exchange_reward_info,
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

    let (route, code) = PacketCodeType::EventExchangeReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
