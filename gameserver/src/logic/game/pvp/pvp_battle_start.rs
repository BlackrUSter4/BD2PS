use bd2::prost::Message;
use bd2::proto::proto_net::{PvpBattleStartRequest, PvpBattleStartResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pvp::pvp_current_match;
use rand::Rng;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(pool: &SqlitePool, uid: i64, req: PvpBattleStartRequest) -> GameResponse {
    info!("Handling PvpBattleStartRequest: {:?}", req);

    let seed = rand::thread_rng().gen_range(i32::MIN..=i32::MAX);
    let _ = pvp_current_match::set_seed(pool, uid, seed).await;

    let response = PvpBattleStartResponse { battle_random_seed: Some(seed) };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::PvpBattleStart.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
