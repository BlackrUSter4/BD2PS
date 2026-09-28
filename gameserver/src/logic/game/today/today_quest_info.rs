use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, QuestDbInfo, TodayQuestInfoRequest, TodayQuestInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::quest::quest_info::get_quest_info;
use database::db::today::today_quest_info::get_today_quest_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, _req: TodayQuestInfoRequest) -> GameResponse {
    info!("Handling TodayQuestInfoRequest for uid: {}", uid);

    let quest_rows = match get_today_quest_info(pool, uid).await {
        Ok(rows) => rows,
        Err(err) => {
            eprintln!("Error fetching TodayQuestInfo: {:?}", err);
            vec![]
        }
    };

    let quest_infos = match get_quest_info(pool, uid).await {
        Ok(rows) => rows,
        Err(err) => {
            eprintln!("Error fetching QuestInfo: {:?}", err);
            vec![]
        }
    };

    let mut clear_quest_ids: Vec<i32> = Vec::new();
    let mut today_quest_ids: Vec<i32> = Vec::new();
    let mut today_end_time: i64 = 0;

    for row in &quest_rows {
        today_quest_ids.push(row.today_quest_id);

        if let Some(id) = row.clear_quest_ids {
            clear_quest_ids.push(id);
        }

        if let Some(end_time) = row.today_end_time {
            today_end_time = end_time;
        }
    }

    let quest_info = quest_infos
        .into_iter()
        .map(|q| QuestDbInfo {
            id: q.id,
            value: q.value,
            object_id: vec![q.object_id],
            quest_level: q.quest_level,
            quest_opt: q.quest_opt,
        })
        .collect::<Vec<_>>();

    let response = TodayQuestInfoResponse {
        quest_info,
        clear_quest_ids,
        today_end_time: Some(today_end_time),
        today_quest_id: today_quest_ids,
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 625, 626, 627],
        active_contents_info: vec![],
        ll_type: Some(String::new()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::TodayQuestInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
