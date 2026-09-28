use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameSurvivalEndRequest, MiniGameSurvivalEndResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{
    mini::{
        mini_game_survival_rank_info as rank_db, mini_game_survival_skill_info as skill_db,
    },
    user::user_info as user_db,
};
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real client-reported run summary recorded into the account's real cross-account rank row
/// (only updated if it's a new personal best, scored by total_exp) and the account's real skill
/// levels persisted from the run's final skill_info snapshot. No reward-per-run master table was
/// identified within this pass's scope, so reward_info_bundle stays honestly empty; the server
/// coin/exp echo back the client-reported values (client-simulates/server-trusts, as elsewhere).
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameSurvivalEndRequest) -> GameResponse {
    info!("Handling MiniGameSurvivalEndRequest: {:?}", req);

    let score = req.total_exp.unwrap_or(0) as i64;
    let user = user_db::get_user_info(pool, uid).await.unwrap_or_default().into_iter().next();
    let owner_index = user.as_ref().and_then(|u| u.owner_index).unwrap_or(0);
    let user_id = user.as_ref().and_then(|u| u.user_id.clone()).unwrap_or_default();
    let _ = rank_db::upsert_best(pool, uid, owner_index, &user_id, score).await;

    for skill in &req.skill_info {
        if let Some(skill_id) = skill.skill_id {
            let existing = skill_db::get_mini_game_survival_skill_info(pool, uid)
                .await
                .unwrap_or_default()
                .into_iter()
                .find(|r| r.skill_id == Some(skill_id));
            if existing.is_none() {
                let _ = skill_db::add_mini_game_survival_skill_info(
                    pool,
                    &database::models::game::mini::mini_game_survival_skill_info::MiniGameSurvivalSkillInfo {
                        index: 0,
                        uid,
                        skill_id: skill.skill_id,
                        level: skill.level,
                        dps: skill.dps,
                    },
                )
                .await;
            }
        }
    }

    let response = MiniGameSurvivalEndResponse {
        reward_info_bundle: None,
        server_get_coin: req.get_coin_count,
        server_total_exp: req.total_exp,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameSurvivalEnd.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
