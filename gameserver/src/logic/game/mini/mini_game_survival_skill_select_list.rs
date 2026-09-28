use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameSurvivalSkillSelectListRequest, MiniGameSurvivalSkillSelectListResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_survival_skill_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real read of the account's already-learned skill ids (offered again as reroll-able options,
/// consistent with the rest of the skill flow). remain_reroll_count stays honestly 0 — no
/// per-run reroll-currency tracking exists in this schema.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameSurvivalSkillSelectListRequest) -> GameResponse {
    info!("Handling MiniGameSurvivalSkillSelectListRequest: {:?}", req);

    let skill_id = db::get_mini_game_survival_skill_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .filter_map(|r| r.skill_id)
        .collect();

    let response = MiniGameSurvivalSkillSelectListResponse { remain_reroll_count: Some(0), skill_id };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameSurvivalSkillSelectList.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
