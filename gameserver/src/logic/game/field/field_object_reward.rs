use crate::logic::field::reward_object::collect_field_object;
use bd2::prost::Message;
use bd2::proto::proto_net::{FieldObjectRewardRequest, FieldObjectRewardResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::{error, info};

pub async fn handle(pool: &SqlitePool, uid: i64, req: FieldObjectRewardRequest) -> GameResponse {
    info!("Handling FieldObjectRewardRequest: {:?}", req);

    // Call the reward collection logic
    let response = match collect_field_object(pool, uid, req).await {
        Ok(resp) => resp,
        Err(e) => {
            error!("Failed to collect field object: {}", e);
            // Return error response
            FieldObjectRewardResponse {
                ..Default::default()
            }
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

    let (route, code) = PacketCodeType::FieldObjectReward.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
