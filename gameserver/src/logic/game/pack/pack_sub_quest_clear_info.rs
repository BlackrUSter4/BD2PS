use bd2::prost::Message;
use bd2::proto::proto_net::{
    PackSubQuestClearInfo, PackSubQuestClearInfoRequest, PackSubQuestClearInfoResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pack::pack_sub_quest_clear_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account sub-quest clear counts, filtered by the requested pack_id list.
pub async fn handle(pool: &SqlitePool, uid: i64, req: PackSubQuestClearInfoRequest) -> GameResponse {
    info!("Handling PackSubQuestClearInfoRequest: {:?}", req);

    let mut sub_quest_clear_info = Vec::new();
    for pack_id in &req.pack_id {
        if let Ok(Some(row)) = db::get_by_uid_and_pack(pool, uid, *pack_id).await {
            sub_quest_clear_info.push(PackSubQuestClearInfo {
                pack_id: row.pack_id,
                sub_quest_clear_count: row.sub_quest_clear_count,
            });
        }
    }

    let response = PackSubQuestClearInfoResponse { sub_quest_clear_info };
    
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
    
    let (route, code) = PacketCodeType::PackSubQuestClearInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}