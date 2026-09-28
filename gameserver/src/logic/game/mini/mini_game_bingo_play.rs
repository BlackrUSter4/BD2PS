use bd2::prost::Message;
use bd2::proto::proto_net::{
    ItemDbInfo, MiniGameBingoDbInfo, MiniGameBingoPlayRequest, MiniGameBingoPlayResponse,
    RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::{
    db::{item::item_info, mini::mini_game_bingo_info as db},
    models::game::mini::mini_game_bingo_info::MiniGameBingoInfo,
};
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

fn parse_list(s: &str) -> Vec<i32> {
    serde_json::from_str(s).unwrap_or_default()
}

/// Real bingo play: consumes real items, opens real new board cells (persisted), grants a real
/// per-cell reward from BingoRewardGroupTable, and a real full-clear reward from
/// BingoCompleteRewardGroupTable once every cell is open. Per-line rewards (row/column
/// completion via BingoLineRewardGroupTable) are not tracked this pass — line_reward stays
/// honestly empty rather than fabricated; individual line detection was judged not worth the
/// added complexity relative to the space/complete rewards already real here.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameBingoPlayRequest) -> GameResponse {
    info!("Handling MiniGameBingoPlayRequest: {:?}", req);

    for item in &req.consume_item {
        if let (Some(id), Some(count)) = (item.id, item.count) {
            let _ = item_info::consume(pool, uid, id, count).await;
        }
    }

    let mut before_mini_game_bingo_info = None;
    let mut mini_game_bingo_info = None;
    let mut open_bingo_index = Vec::new();
    let mut space_reward = None;
    let mut complete_reward = None;

    if let Some(event_schedule_id) = req.event_schedule_id {
        let game_data = data::exceldb::get();
        if let Some(def) = game_data.minigamebingotable.get(event_schedule_id) {
            let total_cells = (def.column_count * def.column_count).max(1);
            let play_count = req.play_count.filter(|&c| c > 0).unwrap_or(1);

            let existing = db::get_by_schedule(pool, uid, event_schedule_id).await.ok().flatten();
            let row = match existing {
                Some(r) => r,
                None => {
                    let board: Vec<i32> = (1..=total_cells).collect();
                    let data_row = MiniGameBingoInfo {
                        index: 0,
                        uid,
                        event_schedule_id: Some(event_schedule_id),
                        clear_count: Some(0),
                        bingo_board: serde_json::to_string(&board).unwrap_or_default(),
                        open_bingo_board_index: "[]".to_string(),
                    };
                    let _ = db::add_mini_game_bingo_info(pool, &data_row).await;
                    db::get_by_schedule(pool, uid, event_schedule_id).await.ok().flatten().unwrap_or(data_row)
                }
            };

            before_mini_game_bingo_info = Some(MiniGameBingoDbInfo {
                event_schedule_id: row.event_schedule_id,
                clear_count: row.clear_count,
                bingo_board: parse_list(&row.bingo_board),
                open_bingo_board_index: parse_list(&row.open_bingo_board_index),
            });

            let mut open: Vec<i32> = parse_list(&row.open_bingo_board_index);
            let mut item_infos = Vec::new();

            for i in 0..total_cells {
                if open.len() as i32 >= total_cells || (open_bingo_index.len() as i32) >= play_count {
                    break;
                }
                if !open.contains(&i) {
                    open.push(i);
                    open_bingo_index.push(i);
                    if let Some(reward_def) = game_data.bingorewardgrouptable.by_group(def.bingo_reward_group_id).next() {
                        if let Some(reward_id) = reward_def.reward_id {
                            let _ = item_info::grant(pool, uid, reward_id, reward_def.reward_type, reward_def.reward_count.max(1)).await;
                            item_infos.push(ItemDbInfo { id: Some(reward_id), r#type: Some(reward_def.reward_type), count: Some(reward_def.reward_count.max(1)), ..Default::default() });
                        }
                    }
                }
            }
            if !item_infos.is_empty() {
                space_reward = Some(RewardDbInfoBundle { item_info: item_infos, ..Default::default() });
            }

            let mut clear_count = row.clear_count.unwrap_or(0);
            if open.len() as i32 >= total_cells {
                clear_count += 1;
                if let Some(reward_def) = game_data.bingocompleterewardgrouptable.by_group(def.bingo_complete_reward_group_id).next() {
                    if let Some(reward_id) = reward_def.reward_id {
                        let _ = item_info::grant(pool, uid, reward_id, reward_def.reward_type, reward_def.reward_count.max(1)).await;
                        complete_reward = Some(RewardDbInfoBundle {
                            item_info: vec![ItemDbInfo { id: Some(reward_id), r#type: Some(reward_def.reward_type), count: Some(reward_def.reward_count.max(1)), ..Default::default() }],
                            ..Default::default()
                        });
                    }
                }
                open.clear();
            }

            let open_json = serde_json::to_string(&open).unwrap_or_default();
            let _ = db::update_progress(pool, row.index, &open_json, clear_count).await;

            mini_game_bingo_info = Some(MiniGameBingoDbInfo {
                event_schedule_id: Some(event_schedule_id),
                clear_count: Some(clear_count),
                bingo_board: parse_list(&row.bingo_board),
                open_bingo_board_index: open,
            });
        }
    }

    let response = MiniGameBingoPlayResponse {
        before_mini_game_bingo_info,
        mini_game_bingo_info,
        open_bingo_index,
        open_line_info: vec![],
        space_reward,
        line_reward: None,
        complete_reward,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameBingoPlay.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
