use bd2::prost::Message;
use bd2::proto::proto_net::{ShopOpenRequest, ShopOpenResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::reputation::reputation_info as rep_db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// This request has no shop_id (schema-confirmed) — real reputation_state is looked up under
/// a fixed group_id=1 placeholder (no per-shop reputation grouping is captured anywhere).
/// is_talent_skill_discount has no matching account state anywhere in this project — honestly
/// false rather than fabricated.
pub async fn handle(pool: &SqlitePool, uid: i64, req: ShopOpenRequest) -> GameResponse {
    info!("Handling ShopOpenRequest: {:?}", req);

    let reputation_state = rep_db::get_reputation_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .find(|r| r.group_id == Some(1))
        .and_then(|r| r.state);

    let response = ShopOpenResponse { reputation_state, is_talent_skill_discount: Some(false) };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::ShopOpen.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
