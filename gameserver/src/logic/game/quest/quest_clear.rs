use crate::logic::field::quest_clear::handle_quest_clear;
use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, QuestClearRequest};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: QuestClearRequest) -> GameResponse {
    info!("Handling QuestClearRequest: {:?}", req);

    let quest_id = req.quest_id.unwrap_or_default();

    let result = handle_quest_clear(pool, uid, quest_id).await;

    let response = match result {
        Ok(resp) => resp,
        Err(err) => {
            tracing::error!("QuestClear failed: {:?}", err);
            return GameResponse::error(404);
        }
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

    let (route, code) = PacketCodeType::QuestClear.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
