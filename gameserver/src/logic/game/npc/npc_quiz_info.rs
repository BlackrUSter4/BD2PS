use bd2::prost::Message;
use bd2::proto::proto_net::{NpcQuizClearDbInfo, NpcQuizInfoRequest, NpcQuizInfoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::npc::npc_quiz_clear_info as db;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: NpcQuizInfoRequest) -> GameResponse {
    info!("Handling NpcQuizInfoRequest: {:?}", req);

    let event_uid = req.event_uid.unwrap_or_default();
    let info = db::get_for_event(pool, uid, event_uid)
        .await
        .into_iter()
        .map(|row| NpcQuizClearDbInfo {
            event_uid: Some(row.event_uid),
            group_id: Some(row.group_id),
            id: Some(row.id),
        })
        .collect();

    let response = NpcQuizInfoResponse { info };
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
    let (route, code) = PacketCodeType::NpcQuizInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
