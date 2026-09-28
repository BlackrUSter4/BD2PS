use bd2::prost::Message;
use bd2::proto::proto_net::{LifeStatSaveRequest, LifeStatSaveResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

/// Client-authoritative save of logging/mining/farming level+exp — matches the request shape
/// exactly, no derived values needed.
pub async fn handle(pool: &SqlitePool, uid: i64, req: LifeStatSaveRequest) -> GameResponse {
    info!("Handling LifeStatSaveRequest: {:?}", req);

    if let Some(level_info) = &req.life_char_level_info {
        let _ = database::db::life::life_user_info::save_char_level(
            pool,
            uid,
            level_info.logging_level.unwrap_or(1),
            level_info.logging_exp.unwrap_or(0),
            level_info.mining_level.unwrap_or(1),
            level_info.mining_exp.unwrap_or(0),
            level_info.farming_level.unwrap_or(1),
            level_info.farming_exp.unwrap_or(0),
        )
        .await;
    }

    let response = LifeStatSaveResponse {};
    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        active_login_event: vec![1, 2, 625, 626, 627],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8818),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::LifeStatSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
