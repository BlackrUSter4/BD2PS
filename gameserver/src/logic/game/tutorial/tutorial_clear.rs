use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, TutorialClearRequest, TutorialClearResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::tutorial::tutorial_info::add_tutorial_info;
use database::models::game::tutorial::tutorial_info::TutorialInfo;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: TutorialClearRequest) -> GameResponse {
    info!(
        "Handling TutorialClearRequest for uid {}: {:?}",
        uid, req.clear_tutorial_id
    );

    if let Some(id) = req.clear_tutorial_id {
        let record = TutorialInfo {
            index: 0, // autoincrement
            uid,
            tutorial_clear_id: id,
        };

        if let Err(e) = add_tutorial_info(pool, &record).await {
            eprintln!(
                "Failed to insert tutorial_clear_id {} for uid {}: {:?}",
                id, uid, e
            );
        }
    }

    let response = TutorialClearResponse {
        ..Default::default()
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::TutorialClear.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
