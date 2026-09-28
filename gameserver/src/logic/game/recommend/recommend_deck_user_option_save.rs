use bd2::prost::Message;
use bd2::proto::proto_net::{RecommendDeckUserOptionSaveRequest, RecommendDeckUserOptionSaveResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::recommend::recommend_deck_user_option_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account display-preference persistence.
pub async fn handle(pool: &SqlitePool, uid: i64, req: RecommendDeckUserOptionSaveRequest) -> GameResponse {
    info!("Handling RecommendDeckUserOptionSaveRequest: {:?}", req);

    if let Some(option) = &req.option_info {
        let _ = db::upsert(pool, uid, option.is_save.unwrap_or(false), option.is_only_char_own_display.unwrap_or(false)).await;
    }

    let response = RecommendDeckUserOptionSaveResponse {};

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

    let (route, code) = PacketCodeType::RecommendDeckUserOptionSave.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
