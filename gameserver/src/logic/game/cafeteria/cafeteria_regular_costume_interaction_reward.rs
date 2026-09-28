use bd2::prost::Message;
use bd2::proto::proto_net::{
    CafeteriaRegularCostumeInteractionRewardRequest, CafeteriaRegularCostumeInteractionRewardResponse,
    CafeteriaRegularCostumeNoteDbInfo, ItemDbInfo, RewardDbInfoBundle,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{cafeteria::cafeteria_regular_costume_note_info as note_db, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: CafeteriaRegularCostumeInteractionRewardRequest,
) -> GameResponse {
    info!("Handling CafeteriaRegularCostumeInteractionRewardRequest: {:?}", req);

    let Some(costume_id) = req.costume_id else {
        return GameResponse::error(1);
    };

    // Real, from `CafeteriaCostumeTable` (keyed by its own row id, not costumeId — linear scan).
    let reward = data::exceldb::get()
        .cafeteriacostumetable
        .all()
        .iter()
        .find(|c| c.costume_id == costume_id)
        .map(|c| (c.reward_id, c.reward_type, c.reward_value));

    if let Some((reward_id, reward_type, reward_value)) = reward {
        let _ = item_info::grant(pool, uid, reward_id, reward_type, reward_value).await;
    }

    let note_row = match note_db::increment_serve_count(pool, uid, costume_id).await {
        Ok(row) => row,
        Err(e) => {
            tracing::error!("CafeteriaRegularCostumeInteractionReward note update failed: {}", e);
            return GameResponse::error(1);
        }
    };

    let response = CafeteriaRegularCostumeInteractionRewardResponse {
        reward_info: reward.map(|(id, ty, count)| RewardDbInfoBundle {
            item_info: vec![ItemDbInfo {
                id: Some(id),
                r#type: Some(ty),
                count: Some(count),
                ..Default::default()
            }],
            ..Default::default()
        }),
        regular_costume_info: Some(CafeteriaRegularCostumeNoteDbInfo {
            costume_id: note_row.costume_id,
            serve_count: note_row.serve_count,
            is_received_reward: note_row.is_received_reward,
        }),
    };

    let resp_bytes = response.encode_to_vec();
    let (route, code) = PacketCodeType::CafeteriaRegularCostumeInteractionReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&super::default_notify())
}
