use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, QuestUpdateRequest, QuestUpdateResponse};
use chrono::Utc;
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::models::game::user::user_quest::UserQuest;
use sqlx::SqlitePool;
use tracing::info;

use data::exceldb::get as get_exceldb;
use database::db::user::user_quest::{add_user_quest, get_user_quest, update_user_quest};

pub async fn handle(pool: &SqlitePool, uid: i64, req: QuestUpdateRequest) -> GameResponse {
    let quest_id = req.quest_id.unwrap_or_default();
    let pack_id = req.pack_id.unwrap_or_default();
    let progress = req.quest_value.get(0).copied().unwrap_or(0);

    info!(
        "QuestUpdateRequest: uid={} quest_id={} pack_id={} progress={}",
        uid, quest_id, pack_id, progress
    );

    let db = get_exceldb();
    let quest_def = match db.questtable1.get(quest_id) {
        Some(q) => q,
        None => return GameResponse::error(404),
    };

    let mut record = match get_user_quest(pool, uid, quest_id).await {
        Ok(Some(q)) => q,
        _ => {
            let q = UserQuest {
                index: 0,
                uid,
                quest_id: quest_id as i64,
                pack_id: Some(pack_id),
                status: 1,
                progress: 0,
                reward_claimed: 0,
                last_update: Some(Utc::now().timestamp_millis()),
            };
            add_user_quest(pool, &q).await.ok();
            q
        }
    };

    record.progress = progress;
    let target = quest_def.condition_count;
    if progress >= target {
        record.status = 2; // Mark as ready-to-clear
    }
    update_user_quest(pool, &record).await.ok();

    let response = QuestUpdateResponse {
        update_quest_id: Some(quest_id),
        reward_info_bundle: None, // client doesn't expect any reward info bundle here
        ..Default::default()
    };
    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8916),
        active_login_event: vec![1, 2],
        ..Default::default()
    };

    let (route, code) = PacketCodeType::QuestUpdate.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
