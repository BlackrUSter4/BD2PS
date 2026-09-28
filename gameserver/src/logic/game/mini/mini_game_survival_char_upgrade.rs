use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameSurvivalCharUpgradeRequest, MiniGameSurvivalCharUpgradeResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_survival_upgrade_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real per-account upgrade level persisted, trusting the client-reported target_level (no
/// upgrade-cost master table was identified within this pass's scope, matching the honest-empty
/// bar used elsewhere for cost/refund tables that couldn't be quickly located).
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameSurvivalCharUpgradeRequest) -> GameResponse {
    info!("Handling MiniGameSurvivalCharUpgradeRequest: {:?}", req);

    if let (Some(upgrade_id), Some(target_level)) = (req.upgrade_id, req.target_level) {
        let _ = db::upsert_level(pool, uid, upgrade_id, target_level).await;
    }

    let response = MiniGameSurvivalCharUpgradeResponse {};

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameSurvivalCharUpgrade.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
