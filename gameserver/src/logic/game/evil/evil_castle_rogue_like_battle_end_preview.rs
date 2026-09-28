use bd2::prost::Message;
use bd2::proto::proto_net::{EvilCastleRogueLikeBattleEndPreviewRequest, EvilCastleRogueLikeBattleEndPreviewResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::evil::evil_castle_rogue_like_info;
use sqlx::SqlitePool;
use tracing::info;

/// Client reports its own battle outcome (result + total_damage, used for
/// the crystal-damage leaderboard stat) — server trusts and persists it,
/// same async pattern as Battle/Colosseum/Ib. Gold preview is a documented
/// placeholder (no reward table keys off battle result specifically).
pub async fn handle(pool: &SqlitePool, uid: i64, req: EvilCastleRogueLikeBattleEndPreviewRequest) -> GameResponse {
    info!("Handling EvilCastleRogueLikeBattleEndPreviewRequest: {:?}", req);

    let won = req.battle_result == Some(1);
    let gold_preview = if won { 100 } else { 0 };

    if let Some(mut run) = evil_castle_rogue_like_info::get_one(pool, uid).await.ok().flatten() {
        if won {
            run.rogue_like_gold = Some(run.rogue_like_gold.unwrap_or(0) + gold_preview);
        }
        if let Some(dmg) = req.total_damage {
            if dmg > run.highest_crystal_damage.unwrap_or(0) {
                run.highest_crystal_damage = Some(dmg);
            }
        }
        let _ = evil_castle_rogue_like_info::upsert(pool, &run).await;
    }

    let response = EvilCastleRogueLikeBattleEndPreviewResponse {
        battle_result: req.battle_result,
        reward_item_preview: vec![],
        rogue_like_gold_preview: Some(gold_preview),
        crystal_damage: req.total_damage,
    };
    let resp_bytes = response.encode_to_vec();
    let notify = Notify { active_login_event: vec![1, 2], ll_type: Some("".to_string()), is_purchasing_disabled: Some(false), maintenance_start_date: Some(1688646600000), ..Default::default() };
    let (route, code) = PacketCodeType::EvilCastleRogueLikeBattleEndPreview.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
