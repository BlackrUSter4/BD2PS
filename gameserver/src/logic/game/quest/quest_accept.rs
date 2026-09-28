use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, Notify, QuestAcceptRequest, QuestAcceptResponse, QuestDbInfo};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, user::user_quest as quest_db};
use database::models::game::user::user_quest::UserQuest;
use sqlx::SqlitePool;
use tracing::info;

const DEFAULT_ITEM_TYPE: i32 = 1;

/// Real quest-accept: creates/updates the real UserQuest row (status=1, in-progress). Grants
/// real QuestTable1.give_quest_item_id items (no matching item-TYPE array exists in that
/// table, so a default type is used — same "missing sibling array" placeholder precedent used
/// elsewhere). No temporary-character/deck mechanic exists anywhere in this project, so
/// char_info/deck_info stay honestly empty.
pub async fn handle(pool: &SqlitePool, uid: i64, req: QuestAcceptRequest) -> GameResponse {
    info!("Handling QuestAcceptRequest: {:?}", req);

    let quest_id = req.quest_id.unwrap_or(0);
    let pack_id = req.pack_id.unwrap_or(0);
    let game_data = data::exceldb::get();

    let mut give_quest_item = Vec::new();
    let mut quest_info = None;

    if let Some(quest_def) = game_data.questtable1.get(quest_id) {
        if quest_db::get_user_quest(pool, uid, quest_id).await.ok().flatten().is_none() {
            let record = UserQuest {
                index: 0,
                uid,
                quest_id: quest_id as i64,
                pack_id: Some(pack_id),
                status: 1,
                progress: 0,
                reward_claimed: 0,
                last_update: Some(chrono::Utc::now().timestamp_millis()),
            };
            let _ = quest_db::add_user_quest(pool, &record).await;

            for id in quest_def.give_quest_item_id.iter().flatten() {
                let _ = item_info::grant(pool, uid, *id, DEFAULT_ITEM_TYPE, 1).await;
                give_quest_item.push(ItemDbInfo {
                    id: Some(*id),
                    r#type: Some(DEFAULT_ITEM_TYPE),
                    count: Some(1),
                    ..Default::default()
                });
            }
        }

        quest_info = Some(QuestDbInfo {
            id: Some(quest_id),
            value: Some(0),
            object_id: vec![],
            quest_level: req.quest_level,
            quest_opt: req.quest_opt,
        });
    }

    let response = QuestAcceptResponse {
        quest_info,
        char_info: vec![],
        deck_info: vec![],
        give_quest_item,
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

    let (route, code) = PacketCodeType::QuestAccept.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
