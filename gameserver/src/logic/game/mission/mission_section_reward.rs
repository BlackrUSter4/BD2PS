use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, MissionSectionRewardRequest, MissionSectionRewardResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{
    item::item_info,
    mission::mission_section_reward_info as reward_db,
};
use database::models::game::mission::mission_section_reward_info::MissionSectionRewardInfo;
use sqlx::SqlitePool;
use tracing::info;

use super::default_notify;

/// Real claim-once section reward against MissionSectionRewardTable's real reward triple.
pub async fn handle(pool: &SqlitePool, uid: i64, req: MissionSectionRewardRequest) -> GameResponse {
    info!("Handling MissionSectionRewardRequest: {:?}", req);

    let group_type = req.group_type.unwrap_or(0);
    let game_data = data::exceldb::get();

    let candidates: Vec<_> = if req.is_all.unwrap_or(false) {
        game_data.missionsectionrewardtable.all().iter().filter(|r| r.group_type == Some(group_type)).collect()
    } else if let Some(id) = req.id {
        game_data.missionsectionrewardtable.get(id).into_iter().collect()
    } else {
        vec![]
    };

    let mut item_info = Vec::new();
    for def in candidates {
        if reward_db::is_claimed(pool, uid, group_type, def.id).await.unwrap_or(false) {
            continue;
        }
        if let Some(reward_id) = def.reward_id {
            let count = def.reward_count.max(1);
            let _ = item_info::grant(pool, uid, reward_id, def.reward_type, count).await;
            item_info.push(ItemDbInfo { id: Some(reward_id), r#type: Some(def.reward_type), count: Some(count), ..Default::default() });
        }
        let _ = reward_db::add_mission_section_reward_info(
            pool,
            &MissionSectionRewardInfo { index: 0, uid, group_type: Some(group_type), id: Some(def.id) },
        )
        .await;
    }

    let response = MissionSectionRewardResponse { item_info, reward_char_info: vec![], costume_info: vec![], equip_info: vec![] };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::MissionSectionReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
