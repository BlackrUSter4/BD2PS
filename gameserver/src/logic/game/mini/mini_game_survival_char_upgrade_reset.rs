use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameSurvivalCharUpgradeResetRequest, MiniGameSurvivalCharUpgradeResetResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_survival_upgrade_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real reset: clears every real upgrade row for the account. No refund-cost master table was
/// identified within this pass's scope (same judgment call as every other refund-shaped handler
/// this session), so reward_info stays honestly empty.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MiniGameSurvivalCharUpgradeResetRequest) -> GameResponse {
    info!("Handling MiniGameSurvivalCharUpgradeResetRequest: {:?}", req);

    let _ = db::delete_mini_game_survival_upgrade_info(pool, uid).await;

    let response = MiniGameSurvivalCharUpgradeResetResponse { reward_info_bundle: None };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MiniGameSurvivalCharUpgradeReset.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
