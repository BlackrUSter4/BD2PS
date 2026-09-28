use bd2::prost::Message;
use bd2::proto::proto_net::{CafeteriaSpawnResetRequest, CafeteriaSpawnResetResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

use super::to_id_list;

pub async fn handle(pool: &SqlitePool, uid: i64, req: CafeteriaSpawnResetRequest) -> GameResponse {
    info!("Handling CafeteriaSpawnResetRequest: {:?}", req);

    let info_row = match super::perform_spawn_reset(pool, uid).await {
        Ok(row) => row,
        Err(e) => {
            tracing::error!("CafeteriaSpawnReset failed: {}", e);
            return GameResponse::error(1);
        }
    };

    let response = CafeteriaSpawnResetResponse {
        daily_connection_costume_id: info_row.daily_connection_costume_id,
        can_get_phone_number: info_row.can_get_phone_number.map(|v| v != 0),
        daily_regular_costume_id: to_id_list(&info_row.daily_regular_costume_ids),
        rewarded_daily_regular_costume_id: to_id_list(&info_row.rewarded_daily_regular_costume_ids),
        daily_npc_reward_currency_count: info_row.daily_npc_reward_currency_count,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CafeteriaSpawnReset.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
