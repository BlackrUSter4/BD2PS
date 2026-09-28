use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, MiniPuzzleOpenRequest, MiniPuzzleOpenResponse, RewardDbInfoBundle, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, mini::mini_puzzle_info as db};
use sqlx::SqlitePool;
use tracing::info;

use super::mini_puzzle_info::to_proto;

/// Real single-cell open against PuzzleRewardGroupTable's real reward (keyed directly by the
/// request's puzzle_reward_id, matching that table's own id column), consuming real items and
/// setting the matching bit in the account's real PuzzleOpen bitmask. Word-completion detection
/// (mini_puzzle_word_reward/word_complete_reward) isn't attempted this pass — left honestly
/// empty.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniPuzzleOpenRequest) -> GameResponse {
    info!("Handling MiniPuzzleOpenRequest: {:?}", req);

    for item in &req.consume_item {
        if let (Some(id), Some(count)) = (item.id, item.count) {
            let _ = item_info::consume(pool, uid, id, count).await;
        }
    }

    let mut before_puzzle_info = None;
    let mut mini_puzzle_info = None;
    let mut open_reward = None;

    if let Some(event_schedule_id) = req.event_schedule_id {
        let row = db::get_or_create(pool, uid, event_schedule_id).await.ok();
        if let Some(row) = row {
            before_puzzle_info = Some(to_proto(&row));

            let mut puzzle_open = row.puzzle_open;
            if let Some(reward_id) = req.puzzle_reward_id {
                let bit = 1i32 << (reward_id % 31);
                puzzle_open |= bit;

                if let Some(def) = data::exceldb::get().puzzlerewardgrouptable.get(reward_id) {
                    if let Some(item_id) = def.reward_id {
                        let ty = def.reward_type.unwrap_or(1);
                        let count = def.reward_count.unwrap_or(1).max(1);
                        let _ = item_info::grant(pool, uid, item_id, ty, count).await;
                        open_reward = Some(RewardDbInfoBundle {
                            item_info: vec![ItemDbInfo { id: Some(item_id), r#type: Some(ty), count: Some(count), ..Default::default() }],
                            ..Default::default()
                        });
                    }
                }
            }

            let _ = db::update_open(pool, row.index, puzzle_open, row.clear_count.unwrap_or(0)).await;
            let mut updated = row;
            updated.puzzle_open = puzzle_open;
            mini_puzzle_info = Some(to_proto(&updated));
        }
    }

    let response = MiniPuzzleOpenResponse {
        before_puzzle_info,
        mini_puzzle_info,
        mini_puzzle_word_reward: None,
        open_reward,
        word_complete_reward: None,
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

    let (route, code) = PacketCodeType::MiniPuzzleOpen.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
