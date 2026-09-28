use bd2::prost::Message;
use bd2::proto::proto_net::{
    MonsterHuntPresetInfoRequest, MonsterHuntPresetInfoResponse, PresetDbInfo, PresetDeckDbInfo,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::monster::monster_hunt_preset_info as db;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: MonsterHuntPresetInfoRequest,
) -> GameResponse {
    info!("Handling MonsterHuntPresetInfoRequest: {:?}", req);

    let rows = db::get_monster_hunt_preset_info(pool, uid).await.unwrap_or_default();
    let preset_info = rows
        .into_iter()
        .map(|r| PresetDbInfo {
            preset_name: r.preset_name,
            preset_resource_id: r.preset_resource_id,
            preset_resource_color: r.preset_resource_color,
            slot: r.slot,
            deck_info: r
                .preset_info_index
                .as_deref()
                .and_then(|s| serde_json::from_str::<Vec<PresetDeckDbInfo>>(s).ok())
                .unwrap_or_default(),
        })
        .collect();

    let response = MonsterHuntPresetInfoResponse { preset_info };
    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MonsterHuntPresetInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
