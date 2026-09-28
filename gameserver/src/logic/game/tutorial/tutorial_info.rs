use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, TutorialInfoRequest, TutorialInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::tutorial::tutorial_info::get_tutorial_info;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: TutorialInfoRequest) -> GameResponse {
    info!("Handling TutorialInfoRequest for uid {}: {:?}", uid, req);

    let tutorial_rows = match get_tutorial_info(pool, uid).await {
        Ok(rows) => rows,
        Err(err) => {
            eprintln!("Error fetching tutorial info for uid {}: {:?}", uid, err);
            vec![]
        }
    };

    let tutorial_clear_id: Vec<i32> = tutorial_rows
        .into_iter()
        .map(|row| row.tutorial_clear_id)
        .collect();

    let response = TutorialInfoResponse { tutorial_clear_id };

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

    let (route, code) = PacketCodeType::TutorialInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
