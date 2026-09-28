use bd2::prost::Message;
use bd2::proto::proto_net::{TotalWarPresetInfoChangeRequest, TotalWarPresetInfoChangeResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::total::total_war_deck_preset_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: TotalWarPresetInfoChangeRequest,
) -> GameResponse {
    info!("Handling TotalWarPresetInfoChangeRequest: {:?}", req);

    let slot = req.slot.unwrap_or(0);
    let _ = db::save_slot(
        pool,
        uid,
        slot,
        req.preset_name.as_deref(),
        req.preset_resource_id,
        req.preset_resource_color,
        None,
    )
    .await;

    let response = TotalWarPresetInfoChangeResponse {};
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::TotalWarPresetInfoChange.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
