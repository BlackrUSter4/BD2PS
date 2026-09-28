use bd2::prost::Message;
use bd2::proto::proto_net::{
    MiniGameSurvivalSkillDbInfo, MiniGameSurvivalSkillUpRequest, MiniGameSurvivalSkillUpResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, mini::mini_game_survival_skill_info as db};
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real per-account skill level-up persisted (level += 1), consuming the real use_item_id when
/// one is given. increase_coin/recovery_hp stay honestly 0 — no skill-up formula/reward table
/// was identified within this pass's scope.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameSurvivalSkillUpRequest) -> GameResponse {
    info!("Handling MiniGameSurvivalSkillUpRequest: {:?}", req);

    if let Some(item_id) = req.use_item_id {
        let _ = item_info::consume(pool, uid, item_id, 1).await;
    }

    if let Some(skill_id) = req.skill_id {
        let current = db::get_mini_game_survival_skill_info(pool, uid)
            .await
            .unwrap_or_default()
            .into_iter()
            .find(|r| r.skill_id == Some(skill_id));
        let next_level = current.as_ref().and_then(|r| r.level).unwrap_or(0) + 1;
        let dps = current.as_ref().and_then(|r| r.dps);
        let _ = db::upsert_level(pool, uid, skill_id, next_level, dps).await;
    }

    let skill_info: Vec<MiniGameSurvivalSkillDbInfo> = db::get_mini_game_survival_skill_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| MiniGameSurvivalSkillDbInfo { skill_id: r.skill_id, level: r.level, dps: r.dps })
        .collect();

    let response = MiniGameSurvivalSkillUpResponse { skill_info, increase_coin: Some(0), recovery_hp: Some(0) };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameSurvivalSkillUp.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
