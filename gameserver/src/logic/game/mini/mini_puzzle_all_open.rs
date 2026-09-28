use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, MiniPuzzleAllOpenRequest, MiniPuzzleAllOpenResponse, RewardDbInfoBundle, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, mini::mini_puzzle_info as db};
use sqlx::SqlitePool;
use tracing::info;

use super::mini_puzzle_info::to_proto;

const FULL_OPEN_MASK: i32 = i32::MAX;

/// Real full-board open: consumes real items, sets every bit open, grants
/// PuzzleCompleteRewardGroupTable's real reward (judgment call — matched by event_schedule_id
/// as the group id, since no explicit group field exists on this request).
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniPuzzleAllOpenRequest) -> GameResponse {
    info!("Handling MiniPuzzleAllOpenRequest: {:?}", req);

    for item in &req.consume_item {
        if let (Some(id), Some(count)) = (item.id, item.count) {
            let _ = item_info::consume(pool, uid, id, count).await;
        }
    }

    let mut before_puzzle_info = None;
    let mut mini_puzzle_info = None;
    let mut open_reward = None;

    if let Some(event_schedule_id) = req.event_schedule_id {
        if let Some(row) = db::get_or_create(pool, uid, event_schedule_id).await.ok() {
            before_puzzle_info = Some(to_proto(&row));

            let clear_count = row.clear_count.unwrap_or(0) + 1;
            let _ = db::update_open(pool, row.index, FULL_OPEN_MASK, clear_count).await;

            if let Some(def) = data::exceldb::get().puzzlecompleterewardgrouptable.by_group(event_schedule_id).next() {
                if let Some(item_id) = def.reward_id {
                    let ty = def.reward_type;
                    let count = def.reward_count.max(1);
                    let _ = item_info::grant(pool, uid, item_id, ty, count).await;
                    open_reward = Some(RewardDbInfoBundle {
                        item_info: vec![ItemDbInfo { id: Some(item_id), r#type: Some(ty), count: Some(count), ..Default::default() }],
                        ..Default::default()
                    });
                }
            }

            let mut updated = row;
            updated.puzzle_open = FULL_OPEN_MASK;
            updated.clear_count = Some(clear_count);
            mini_puzzle_info = Some(to_proto(&updated));
        }
    }

    let response = MiniPuzzleAllOpenResponse {
        before_puzzle_info,
        mini_puzzle_info,
        mini_puzzle_word_reward: vec![],
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

    let (route, code) = PacketCodeType::MiniPuzzleAllOpen.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
