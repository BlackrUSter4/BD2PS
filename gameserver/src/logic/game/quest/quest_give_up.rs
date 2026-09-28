use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, Notify, QuestGiveUpRequest, QuestGiveUpResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, user::user_quest as quest_db};
use sqlx::SqlitePool;
use tracing::info;

const DEFAULT_ITEM_TYPE: i32 = 1;

/// Real quest give-up: deletes the account's real UserQuest row (so the quest can be
/// re-accepted) and reverses the real quest-start items granted by QuestAccept. No temporary
/// character mechanic exists anywhere in this project, so delete_char_inven_index stays
/// honestly empty.
pub async fn handle(pool: &SqlitePool, uid: i64, req: QuestGiveUpRequest) -> GameResponse {
    info!("Handling QuestGiveUpRequest: {:?}", req);

    let quest_id = req.quest_id.unwrap_or(0);
    let game_data = data::exceldb::get();

    let mut remove_quest_item = Vec::new();
    if let Some(quest_def) = game_data.questtable1.get(quest_id) {
        for id in quest_def.give_quest_item_id.iter().flatten() {
            if item_info::consume(pool, uid, *id, 1).await.unwrap_or(false) {
                remove_quest_item.push(ItemDbInfo {
                    id: Some(*id),
                    r#type: Some(DEFAULT_ITEM_TYPE),
                    count: Some(1),
                    ..Default::default()
                });
            }
        }
    }

    let _ = quest_db::delete_user_quest(pool, uid, quest_id).await;

    let response = QuestGiveUpResponse {
        quest_id: Some(quest_id),
        deck_info: vec![],
        remove_quest_item,
        delete_char_inven_index: vec![],
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::QuestGiveUp.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
