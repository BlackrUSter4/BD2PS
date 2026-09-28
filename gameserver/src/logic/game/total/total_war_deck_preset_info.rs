use bd2::prost::Message;
use bd2::proto::proto_net::{
    TotalWarDeckDbInfo, TotalWarDeckPresetDbInfo, TotalWarDeckPresetInfoRequest,
    TotalWarDeckPresetInfoResponse,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::total::total_war_deck_preset_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: TotalWarDeckPresetInfoRequest,
) -> GameResponse {
    info!("Handling TotalWarDeckPresetInfoRequest: {:?}", req);

    let rows = db::get_total_war_deck_preset_info(pool, uid).await.unwrap_or_default();
    let preset_info = rows
        .into_iter()
        .map(|r| TotalWarDeckPresetDbInfo {
            slot: r.slot,
            preset_name: r.preset_name,
            resource_id: r.resource_id,
            resource_color: r.resource_color,
            deck_info: r
                .deck_info_index
                .as_deref()
                .and_then(|s| serde_json::from_str::<Vec<TotalWarDeckDbInfo>>(s).ok())
                .unwrap_or_default(),
        })
        .collect();

    let response = TotalWarDeckPresetInfoResponse { preset_info };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::TotalWarDeckPresetInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
