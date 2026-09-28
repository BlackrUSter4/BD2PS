use bd2::prost::Message;
use bd2::proto::proto_net::{
    ItemDbInfo, MiniGameRouletteDbInfo, MiniGameRouletteDrawRequest, MiniGameRouletteDrawResponse,
    MiniGameRouletteRewardInfo as MiniGameRouletteRewardInfoProto, RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::{
    db::{item::item_info, mini::{mini_game_roulette_info as db, mini_game_roulette_reward_info as reward_db}},
    models::game::mini::{mini_game_roulette_info::MiniGameRouletteInfo, mini_game_roulette_reward_info::MiniGameRouletteRewardInfo},
};
use sqlx::SqlitePool;
use tracing::info;

/// Real weighted draw against RouletteRewardGroupTable's real `probability` weights, real
/// accumulated-count bonus from RouletteAccumulatedRewardTable once the exact threshold is
/// hit, real consumption of the client-reported cost.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameRouletteDrawRequest) -> GameResponse {
    info!("Handling MiniGameRouletteDrawRequest: {:?}", req);

    for item in &req.consume_item {
        if let (Some(id), Some(count)) = (item.id, item.count) {
            let _ = item_info::consume(pool, uid, id, count).await;
        }
    }

    let mut roulette_info = None;
    let mut roulette_reward = None;
    let mut roulette_accumulated_reward = None;
    let mut roulette_reward_info = Vec::new();

    if let Some(event_schedule_id) = req.event_schedule_id {
        let game_data = data::exceldb::get();
        if let Some(def) = game_data.minigameroulettetable.get(event_schedule_id) {
            let draw_count = req.draw_count.filter(|&c| c > 0).unwrap_or(1);

            let row = match db::get_by_schedule(pool, uid, event_schedule_id).await.ok().flatten() {
                Some(r) => r,
                None => {
                    let new_row = MiniGameRouletteInfo {
                        index: 0,
                        uid,
                        event_schedule_id: Some(event_schedule_id),
                        free_ap_count: Some(def.free_count_day),
                        reset_time: Some(chrono::Utc::now().timestamp_millis() + 86_400_000),
                        is_reward_special_item: Some(false),
                        try_count: Some(0),
                    };
                    let _ = db::add_mini_game_roulette_info(pool, &new_row).await;
                    db::get_by_schedule(pool, uid, event_schedule_id).await.ok().flatten().unwrap_or(new_row)
                }
            };

            let candidates: Vec<_> = game_data.rouletterewardgrouptable.by_group(def.roulette_reward_group_id).collect();
            let total_weight: i32 = candidates.iter().map(|c| c.probability.unwrap_or(1)).sum::<i32>().max(1);

            let mut item_infos = Vec::new();
            let mut try_count = row.try_count.unwrap_or(0);

            for _ in 0..draw_count {
                if candidates.is_empty() {
                    break;
                }
                let mut roll = (chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0).unsigned_abs() % total_weight as u64) as i32;
                let mut chosen = candidates[0];
                for c in &candidates {
                    let weight = c.probability.unwrap_or(1);
                    if roll < weight {
                        chosen = c;
                        break;
                    }
                    roll -= weight;
                }

                if let Some(reward_id) = chosen.reward_id {
                    let _ = item_info::grant(pool, uid, reward_id, chosen.reward_type, chosen.reward_count.max(1)).await;
                    item_infos.push(ItemDbInfo { id: Some(reward_id), r#type: Some(chosen.reward_type), count: Some(chosen.reward_count.max(1)), ..Default::default() });
                }
                let _ = reward_db::add_mini_game_roulette_reward_info(pool, &MiniGameRouletteRewardInfo { index: 0, uid, group_id: Some(def.roulette_reward_group_id), id: Some(chosen.id) }).await;
                roulette_reward_info.push(MiniGameRouletteRewardInfoProto { group_id: Some(def.roulette_reward_group_id), id: Some(chosen.id) });

                try_count += 1;
            }
            if !item_infos.is_empty() {
                roulette_reward = Some(RewardDbInfoBundle { item_info: item_infos, ..Default::default() });
            }

            if let Some(acc_def) = game_data.rouletteaccumulatedrewardtable.by_group(def.roulette_accumulated_reward_group_id).find(|a| a.accumulated_count == try_count) {
                if let Some(reward_id) = acc_def.reward_id {
                    let _ = item_info::grant(pool, uid, reward_id, acc_def.reward_type, acc_def.reward_count.max(1)).await;
                    roulette_accumulated_reward = Some(RewardDbInfoBundle {
                        item_info: vec![ItemDbInfo { id: Some(reward_id), r#type: Some(acc_def.reward_type), count: Some(acc_def.reward_count.max(1)), ..Default::default() }],
                        ..Default::default()
                    });
                }
            }

            let _ = db::update_try_count(pool, row.index, try_count).await;

            roulette_info = Some(MiniGameRouletteDbInfo {
                event_schedule_id: Some(event_schedule_id),
                free_ap_count: row.free_ap_count,
                reset_time: row.reset_time,
                is_reward_special_item: row.is_reward_special_item,
                try_count: Some(try_count),
            });
        }
    }

    let response = MiniGameRouletteDrawResponse { roulette_info, roulette_reward, roulette_accumulated_reward, roulette_reward_info };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameRouletteDraw.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
