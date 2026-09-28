use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeQuickBattleRequest, EvilCastleRogueLikeQuickBattleResponse, EvilCastleRogueLikeScoreInfo as ScoreInfoMsg, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use data::exceldb;
use database::db::evil::evil_castle_rogue_like_score_info;
use database::models::game::evil::evil_castle_rogue_like_score_info::EvilCastleRogueLikeScoreInfo;
use sqlx::SqlitePool;
use tracing::info;

/// "Skip straight to results" for a level the account has already cleared
/// before — real score accumulation using RLLevelTable's real per-level
/// score_bonus_rate, multiplied by the claimed clear_count.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeQuickBattleRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeQuickBattleRequest: {:?}", req);

    let level = req.level.unwrap_or(1);
    let clear_count = req.clear_count.unwrap_or(1).max(1);
    let bonus_rate = exceldb::get().rlleveltable.get(level).and_then(|l| l.score_bonus_rate).unwrap_or(0.1);
    let gained = ((clear_count as f32) * bonus_rate * 1_000_000.0) as i64;

    let mut score = evil_castle_rogue_like_score_info::get_one(pool, uid).await.ok().flatten().unwrap_or(EvilCastleRogueLikeScoreInfo {
        index: 0, uid, total_score: Some(0), obsidian: Some(0), score_item_info_index: None, all_user_total_score: None, max_try_level: Some(0), max_reward_level: Some(0), crystal_damage: Some(0),
    });
    score.total_score = Some(score.total_score.unwrap_or(0) + gained as i32);
    score.max_try_level = Some(score.max_try_level.unwrap_or(0).max(level));
    let _ = evil_castle_rogue_like_score_info::upsert(pool, &score).await;

    let response = EvilCastleRogueLikeQuickBattleResponse {
        score_info: Some(ScoreInfoMsg {
            score_item_info: vec![],
            total_score: score.total_score,
            obsidian: score.obsidian,
            all_user_total_score: evil_castle_rogue_like_score_info::sum_all_user_total_score(pool).await.ok(),
            max_try_level: score.max_try_level,
            max_reward_level: score.max_reward_level,
            crystal_damage: score.crystal_damage.map(|v| v as i64),
        }),
    };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeQuickBattle.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
