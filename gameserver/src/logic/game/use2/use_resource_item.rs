use bd2::prost::Message;
use bd2::proto::proto_net::{RewardDbInfoBundle, UseResourceItemRequest, UseResourceItemResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

use super::{default_notify, grant_reward_group_choice};

/// Real player-choice item use: consumes the real owned item, grants the client's chosen
/// `select_value` slot from RewardGroupTable (no dedicated "resource item" master table
/// distinct from RandomBoxTable exists, so the same item->reward_group_id lookup is reused —
/// judgment call, since it's the only table linking an item id to a reward group in this
/// project).
pub async fn handle(pool: &SqlitePool, uid: i64, req: UseResourceItemRequest) -> GameResponse {
    info!("Handling UseResourceItemRequest: {:?}", req);

    let mut item_infos = Vec::new();
    if let (Some(inven_index), Some(select_value)) = (req.inven_index, req.select_value) {
        let use_count = req.use_count.filter(|&c| c > 0).unwrap_or(1);
        if let Ok(Some(item)) = item_info::get_by_inven_index(pool, uid, inven_index).await {
            if let Some(item_id) = item.id {
                if item_info::reduce_by_inven_index(pool, uid, inven_index, use_count).await.unwrap_or(false) {
                    if let Some(def) = data::exceldb::get().randomboxtable.get(item_id) {
                        let reward_group_id = def.reward_group_id;
                        for _ in 0..use_count {
                            if let Some(reward) = grant_reward_group_choice(pool, uid, reward_group_id, select_value as usize).await {
                                item_infos.push(reward);
                            }
                        }
                    }
                }
            }
        }
    }

    let response = UseResourceItemResponse {
        reward_info_bundle: Some(RewardDbInfoBundle { item_info: item_infos, ..Default::default() }),
        first_auto_revive_set_char_inven_index: None,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::UseResourceItem.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&default_notify())
}
