use bd2::prost::Message;
use bd2::proto::proto_net::{
    CafeteriaRegularCostumeNoteAllRewardRequest, CafeteriaRegularCostumeNoteAllRewardResponse,
    ItemDbInfo, RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{cafeteria::cafeteria_regular_costume_note_info as note_db, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: CafeteriaRegularCostumeNoteAllRewardRequest,
) -> GameResponse {
    info!("Handling CafeteriaRegularCostumeNoteAllRewardRequest: {:?}", req);

    let Some(default) = data::exceldb::get().cafeteriadefaulttable.all().first() else {
        return GameResponse::error(1);
    };
    let threshold = default.cafeteria_note_condition_value;
    let reward_value = default.note_reward_value;

    let notes = note_db::get_cafeteria_regular_costume_note_info(pool, uid)
        .await
        .unwrap_or_default();

    let mut reward_received_costume_id = Vec::new();
    let mut item_infos = Vec::new();

    for note in notes {
        let Some(costume_id) = note.costume_id else { continue };
        let eligible =
            note.serve_count.unwrap_or(0) >= threshold && !note.is_received_reward.unwrap_or(false);
        if !eligible {
            continue;
        }
        let _ = item_info::grant(pool, uid, super::GOLD_ITEM_ID, super::GOLD_ITEM_TYPE, reward_value)
            .await;
        let _ = note_db::mark_reward_received(pool, uid, costume_id).await;
        reward_received_costume_id.push(costume_id);
        item_infos.push(ItemDbInfo {
            id: Some(super::GOLD_ITEM_ID),
            r#type: Some(super::GOLD_ITEM_TYPE),
            count: Some(reward_value),
            ..Default::default()
        });
    }

    let response = CafeteriaRegularCostumeNoteAllRewardResponse {
        reward_info: Some(RewardDbInfoBundle {
            item_info: item_infos,
            ..Default::default()
        }),
        reward_received_costume_id,
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CafeteriaRegularCostumeNoteAllReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
