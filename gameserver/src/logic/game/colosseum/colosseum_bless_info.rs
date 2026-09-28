use bd2::prost::Message;
use bd2::proto::proto_net::{ColosseumBlessInfoRequest, ColosseumBlessInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::colosseum::colosseum_bless_info;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(pool: &SqlitePool, uid: i64, req: ColosseumBlessInfoRequest) -> GameResponse {
    info!("Handling ColosseumBlessInfoRequest: {:?}", req);

    let rows = colosseum_bless_info::get_by_uid(pool, uid).await.unwrap_or_default();
    let attack_id = rows.iter().filter(|r| r.deck_type == 0).map(|r| r.bless_id).collect();
    let defense_id = rows.iter().filter(|r| r.deck_type == 1).map(|r| r.bless_id).collect();

    let response = ColosseumBlessInfoResponse { attack_id, defense_id };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ColosseumBlessInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
