use bd2::prost::Message;
use bd2::proto::proto_net::{MiniGameDefenseInfoRequest, MiniGameDefenseInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::mini::mini_game_defense_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real read (was hardcoding a fake event_schedule_id for every account despite a real,
/// already-scaffolded table sitting unused).
pub async fn handle(
    pool: &SqlitePool,
    uid: i64,
    req: MiniGameDefenseInfoRequest,
) -> GameResponse {
    info!("Handling MiniGameDefenseInfoRequest: {:?}", req);

    let rows = db::get_mini_game_defense_info(pool, uid).await.unwrap_or_default();
    let event_schedule_id = rows.first().and_then(|r| r.event_schedule_id);
    let reward_info = rows.iter().map(|r| r.reward_info).collect();

    let response = MiniGameDefenseInfoResponse { event_schedule_id, reward_info };

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

    let (route, code) = PacketCodeType::MiniGameDefenseInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
