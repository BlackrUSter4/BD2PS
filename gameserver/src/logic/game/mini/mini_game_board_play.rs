use bd2::prost::Message;
use bd2::proto::proto_net::{
    ItemDbInfo, MiniGameBoardDbInfo, MiniGameBoardPlayRequest, MiniGameBoardPlayResponse,
    RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, mini::mini_game_board_info as db};
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;
use crate::logic::game::use2::roll_reward_group;

/// Real board advance against MiniGameBoardTable/MiniGameScaffoldTable's real per-scaffold
/// item cost and reward, using MiniGameBoardTable.mini_game_complete_reward_group_id (via the
/// shared RewardGroupTable weighted-roll helper) once the whole scaffold group is cleared.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameBoardPlayRequest) -> GameResponse {
    info!("Handling MiniGameBoardPlayRequest: {:?}", req);

    for item in &req.consume_item {
        if let (Some(id), Some(count)) = (item.id, item.count) {
            let _ = item_info::consume(pool, uid, id, count).await;
        }
    }

    let mut mini_game_board_info = None;
    let mut scaffold_reward = None;
    let mut complete_reward = None;

    if let Some(event_schedule_id) = req.event_schedule_id {
        let game_data = data::exceldb::get();
        if let Some(board_def) = game_data.minigameboardtable.get(event_schedule_id) {
            let scaffolds: Vec<_> = game_data.minigamescaffoldtable.by_group(board_def.scaffold_group_id).collect();
            let row = db::get_by_schedule(pool, uid, event_schedule_id).await.ok().flatten();

            let current_scaffold_id = row.as_ref().and_then(|r| r.scaffold_id);
            let next_index = match current_scaffold_id {
                Some(current) => scaffolds.iter().position(|s| s.id == current).map(|p| p + 1).unwrap_or(0),
                None => 0,
            };

            let mut complete_count = row.as_ref().and_then(|r| r.complete_count).unwrap_or(0);
            let new_scaffold_id;

            if next_index < scaffolds.len() {
                let scaffold = scaffolds[next_index];
                new_scaffold_id = scaffold.id;
                if let Some(item_id) = scaffold.item_id {
                    let _ = item_info::grant(pool, uid, item_id, scaffold.item_type, scaffold.item_count.max(1)).await;
                    scaffold_reward = Some(RewardDbInfoBundle {
                        item_info: vec![ItemDbInfo { id: Some(item_id), r#type: Some(scaffold.item_type), count: Some(scaffold.item_count.max(1)), ..Default::default() }],
                        ..Default::default()
                    });
                }
                if next_index == scaffolds.len() - 1 {
                    complete_count += 1;
                    if let Some(reward) = roll_reward_group(pool, uid, board_def.mini_game_complete_reward_group_id).await {
                        complete_reward = Some(RewardDbInfoBundle { item_info: vec![reward], ..Default::default() });
                    }
                }
            } else {
                new_scaffold_id = scaffolds.first().map(|s| s.id).unwrap_or(0);
            }

            let _ = db::upsert_progress(pool, uid, event_schedule_id, board_def.scaffold_group_id, new_scaffold_id, complete_count).await;

            mini_game_board_info = Some(MiniGameBoardDbInfo {
                event_schedule_id: Some(event_schedule_id),
                scaffold_group_id: Some(board_def.scaffold_group_id),
                scaffold_id: Some(new_scaffold_id),
                complete_count: Some(complete_count),
            });
        }
    }

    let response = MiniGameBoardPlayResponse {
        mini_game_board_info,
        mini_game_controller_info: vec![],
        complete_reward,
        scaffold_reward,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameBoardPlay.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}

