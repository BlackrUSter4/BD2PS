use bd2::prost::Message;
use bd2::proto::proto_net::{
    CafeteriaRegularCostumeNoteRewardRequest, CafeteriaRegularCostumeNoteRewardResponse, ItemDbInfo,
    RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{cafeteria::cafeteria_regular_costume_note_info as note_db, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

/// Threshold is real (`CafeteriaDefaultTable.cafeteriaNoteConditionValue`); the reward AMOUNT is
/// real too (`noteRewardValue`), but no table names an actual item id for `noteRewardType`, so —
/// same convention as every other round with this gap — it's granted as gold.
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: CafeteriaRegularCostumeNoteRewardRequest,
) -> GameResponse {
    info!("Handling CafeteriaRegularCostumeNoteRewardRequest: {:?}", req);

    let Some(costume_id) = req.costume_id else {
        return GameResponse::error(1);
    };

    let Some(default) = data::exceldb::get().cafeteriadefaulttable.all().first() else {
        return GameResponse::error(1);
    };
    let threshold = default.cafeteria_note_condition_value;
    let reward_value = default.note_reward_value;

    let mut reward_info = None;

    if let Ok(Some(note_row)) = note_db::get_by_costume_id(pool, uid, costume_id).await {
        let eligible = note_row.serve_count.unwrap_or(0) >= threshold
            && !note_row.is_received_reward.unwrap_or(false);
        if eligible {
            let _ = item_info::grant(pool, uid, super::GOLD_ITEM_ID, super::GOLD_ITEM_TYPE, reward_value)
                .await;
            let _ = note_db::mark_reward_received(pool, uid, costume_id).await;
            reward_info = Some(RewardDbInfoBundle {
                item_info: vec![ItemDbInfo {
                    id: Some(super::GOLD_ITEM_ID),
                    r#type: Some(super::GOLD_ITEM_TYPE),
                    count: Some(reward_value),
                    ..Default::default()
                }],
                ..Default::default()
            });
        }
    }

    let response = CafeteriaRegularCostumeNoteRewardResponse {
        reward_info: Some(reward_info.unwrap_or_default()),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CafeteriaRegularCostumeNoteReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
