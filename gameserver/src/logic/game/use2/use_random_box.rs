use bd2::prost::Message;
use bd2::proto::proto_net::{RewardDbInfoBundle, UseRandomBoxRequest, UseRandomBoxResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, roll_reward_group};

/// Real box-open: consumes the real owned box item, real-weighted-rolls its
/// RandomBoxTable.reward_group_id against RewardGroupTable's real ratio array.
pub async fn handle(pool: &SqlitePool, uid: i64, req: UseRandomBoxRequest) -> GameResponse {
    info!("Handling UseRandomBoxRequest: {:?}", req);

    let mut item_infos = Vec::new();
    if let Some(inven_index) = req.inven_index {
        let use_count = req.use_count.filter(|&c| c > 0).unwrap_or(1);
        if let Ok(Some(item)) = item_info::get_by_inven_index(pool, uid, inven_index).await {
            if let Some(box_id) = item.id {
                if item_info::reduce_by_inven_index(pool, uid, inven_index, use_count).await.unwrap_or(false) {
                    if let Some(def) = data::exceldb::get().randomboxtable.get(box_id) {
                        let reward_group_id = def.reward_group_id;
                        for _ in 0..use_count {
                            if let Some(reward) = roll_reward_group(pool, uid, reward_group_id).await {
                                item_infos.push(reward);
                            }
                        }
                    }
                }
            }
        }
    }

    let response = UseRandomBoxResponse {
        reward_info_bundle: Some(RewardDbInfoBundle { item_info: item_infos, ..Default::default() }),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::UseRandomBox.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
