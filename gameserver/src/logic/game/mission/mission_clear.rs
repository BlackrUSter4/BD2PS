use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, MissionClearRequest, MissionClearResponse, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, mission::mission_info as mission_db};
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real claim: only already-real-complete MissionInfo rows are claimable (set by
/// mission_update against MissionTable's real condition_value), granting each mission's real
/// reward_id/type/count and removing the row so it can't be claimed twice.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MissionClearRequest) -> GameResponse {
    info!("Handling MissionClearRequest: {:?}", req);

    let game_data = data::exceldb::get();
    let completed = mission_db::get_completed(pool, uid).await.unwrap_or_default();

    let targets: Vec<_> = completed
        .into_iter()
        .filter(|m| {
            if req.is_all.unwrap_or(false) {
                req.group_type.is_none() || m.group_type == req.group_type
            } else {
                m.group_id == req.group_id && m.id == req.id
            }
        })
        .collect();

    let mut item_infos = Vec::new();
    for m in &targets {
        if let Some(id) = m.id {
            if let Some(def) = game_data.missiontable.get(id) {
                if let Some(reward_id) = def.reward_id {
                    let ty = def.reward_type.unwrap_or(1);
                    let count = def.reward_count.unwrap_or(0).max(1);
                    let _ = item_info::grant(pool, uid, reward_id, ty, count).await;
                    item_infos.push(ItemDbInfo { id: Some(reward_id), r#type: Some(ty), count: Some(count), ..Default::default() });
                }
            }
        }
        if let (Some(group_id), Some(id)) = (m.group_id, m.id) {
            let _ = mission_db::delete_by_group_and_id(pool, uid, group_id, id).await;
        }
    }

    let response = MissionClearResponse {
        reward_info_bundle: Some(RewardDbInfoBundle { item_info: item_infos, ..Default::default() }),
        first_auto_revive_set_char_inven_index: None,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MissionClear.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
