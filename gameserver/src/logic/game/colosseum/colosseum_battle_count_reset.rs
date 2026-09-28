use bd2::prost::Message;
use bd2::proto::proto_net::{ColosseumBattleCountResetRequest, ColosseumBattleCountResetResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::colosseum::colosseum_user_info;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, now_ms};

pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumBattleCountResetRequest) -> GameResponse {
    info!("Handling ColosseumBattleCountResetRequest: {:?}", req);

    let now = now_ms();
    let _ = colosseum_user_info::set_battle_count_reset(pool, uid, now).await;

    let response = ColosseumBattleCountResetResponse { reset_time: Some(now) };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumBattleCountReset.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
