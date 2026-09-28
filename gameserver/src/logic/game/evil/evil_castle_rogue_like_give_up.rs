use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeGiveUpRequest, EvilCastleRogueLikeGiveUpResponse, EvilCastleRogueLikeScoreInfo as ScoreInfoMsg, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::{
    evil_castle_rogue_like_deck_info, evil_castle_rogue_like_event_info, evil_castle_rogue_like_floor_info,
    evil_castle_rogue_like_info, evil_castle_rogue_like_room_info, evil_castle_rogue_like_score_info,
    evil_castle_rogue_like_shop_info, evil_castle_rogue_like_shop_item_info, evil_castle_rogue_like_state_info,
};
use database::models::game::evil::evil_castle_rogue_like_score_info::EvilCastleRogueLikeScoreInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Ends the current run: persists the final score/obsidian tally for real
/// (cross-account ranking reads this), then wipes all run-transient state.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeGiveUpRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeGiveUpRequest: {:?}", req);
    let _ = req;

    let run = evil_castle_rogue_like_info::get_one(pool, uid).await.ok().flatten();
    let score_row = EvilCastleRogueLikeScoreInfo {
        index: 0,
        uid,
        score_item_info_index: None,
        total_score: run.as_ref().and_then(|r| r.rogue_like_gold),
        obsidian: run.as_ref().and_then(|r| r.obsidian),
        all_user_total_score: None,
        max_try_level: run.as_ref().and_then(|r| r.max_try_level),
        max_reward_level: run.as_ref().and_then(|r| r.max_reward_level),
        crystal_damage: run.as_ref().and_then(|r| r.highest_crystal_damage),
    };
    let _ = evil_castle_rogue_like_score_info::upsert(pool, &score_row).await;

    let _ = evil_castle_rogue_like_info::delete_evil_castle_rogue_like_info(pool, uid).await;
    let _ = evil_castle_rogue_like_state_info::delete_evil_castle_rogue_like_state_info(pool, uid).await;
    let _ = evil_castle_rogue_like_floor_info::delete_evil_castle_rogue_like_floor_info(pool, uid).await;
    let _ = evil_castle_rogue_like_room_info::delete_evil_castle_rogue_like_room_info(pool, uid).await;
    let _ = evil_castle_rogue_like_deck_info::delete(pool, uid).await;
    let _ = evil_castle_rogue_like_event_info::delete_evil_castle_rogue_like_event_info(pool, uid).await;
    let _ = evil_castle_rogue_like_shop_info::delete_evil_castle_rogue_like_shop_info(pool, uid).await;
    let _ = evil_castle_rogue_like_shop_item_info::delete_evil_castle_rogue_like_shop_item_info(pool, uid).await;

    let response = EvilCastleRogueLikeGiveUpResponse {
        score_info: Some(ScoreInfoMsg {
            score_item_info: vec![],
            total_score: score_row.total_score,
            obsidian: score_row.obsidian,
            all_user_total_score: evil_castle_rogue_like_score_info::sum_all_user_total_score(pool).await.ok(),
            max_try_level: score_row.max_try_level,
            max_reward_level: score_row.max_reward_level,
            crystal_damage: score_row.crystal_damage.map(|v| v as i64),
        }),
    };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeGiveUp.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
