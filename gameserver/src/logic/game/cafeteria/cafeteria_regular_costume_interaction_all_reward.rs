use bd2::prost::Message;
use bd2::proto::proto_net::{
    CafeteriaRegularCostumeInteractionAllRewardRequest,
    CafeteriaRegularCostumeInteractionAllRewardResponse, CafeteriaRegularCostumeNoteDbInfo,
    ItemDbInfo, RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{
    cafeteria::{cafeteria_info, cafeteria_regular_costume_note_info as note_db},
    item::item_info,
};
use sqlx::SqlitePool;
use tracing::info;

use super::{from_id_list, to_id_list};

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: CafeteriaRegularCostumeInteractionAllRewardRequest,
) -> GameResponse {
    info!("Handling CafeteriaRegularCostumeInteractionAllRewardRequest: {:?}", req);

    let mut info_row = match cafeteria_info::get_or_create(pool, uid).await {
        Ok(row) => row,
        Err(e) => {
            tracing::error!("CafeteriaRegularCostumeInteractionAllReward get_or_create failed: {}", e);
            return GameResponse::error(1);
        }
    };

    let on_shift = to_id_list(&info_row.daily_regular_costume_ids);
    let already_rewarded = to_id_list(&info_row.rewarded_daily_regular_costume_ids);

    let table = data::exceldb::get().cafeteriacostumetable.all();

    let mut rewarded_costume_id = Vec::new();
    let mut item_infos = Vec::new();
    let mut notes = Vec::new();
    let mut new_rewarded = already_rewarded.clone();

    for costume_id in &on_shift {
        if already_rewarded.contains(costume_id) {
            continue;
        }
        if let Some(c) = table.iter().find(|c| c.costume_id == *costume_id) {
            let _ = item_info::grant(pool, uid, c.reward_id, c.reward_type, c.reward_value).await;
            item_infos.push(ItemDbInfo {
                id: Some(c.reward_id),
                r#type: Some(c.reward_type),
                count: Some(c.reward_value),
                ..Default::default()
            });
        }
        let note_row = note_db::increment_serve_count(pool, uid, *costume_id)
            .await
            .ok();
        if let Some(n) = note_row {
            notes.push(CafeteriaRegularCostumeNoteDbInfo {
                costume_id: n.costume_id,
                serve_count: n.serve_count,
                is_received_reward: n.is_received_reward,
            });
        }
        rewarded_costume_id.push(*costume_id);
        new_rewarded.push(*costume_id);
    }

    info_row.rewarded_daily_regular_costume_ids = from_id_list(&new_rewarded);
    if let Err(e) = cafeteria_info::update_cafeteria_info(pool, &info_row).await {
        tracing::error!("CafeteriaRegularCostumeInteractionAllReward update failed: {}", e);
    }

    let response = CafeteriaRegularCostumeInteractionAllRewardResponse {
        rewarded_costume_id,
        reward_info: Some(RewardDbInfoBundle {
            item_info: item_infos,
            ..Default::default()
        }),
        regular_costume_info: notes,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CafeteriaRegularCostumeInteractionAllReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
