use bd2::prost::Message;
use bd2::proto::proto_net::{
    PackRewardObjectCountInfo, PackRewardObjectCountRequest, PackRewardObjectCountResponse, Notify,
};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::pack::pack_reward_object_count_info as db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account (pack_id, count_type) counters. Judgment call: `max_count` has no
/// backing table anywhere in this project — a flat placeholder (3) is used only for
/// brand-new counters; an existing counter's stored MaxCount always wins.
const PLACEHOLDER_MAX_COUNT: i32 = 3;

pub async fn handle(pool: &SqlitePool, uid: i64, req: PackRewardObjectCountRequest) -> GameResponse {
    info!("Handling PackRewardObjectCountRequest: {:?}", req);

    let mut count_info = Vec::new();
    for i in 0..req.pack_id.len() {
        let pack_id = req.pack_id[i];
        let ty = req.count_type.get(i).copied().unwrap_or(0);
        if let Ok(row) = db::get_or_create(pool, uid, pack_id, ty, PLACEHOLDER_MAX_COUNT).await {
            count_info.push(PackRewardObjectCountInfo {
                r#type: row.r#type,
                pack_id: row.pack_id,
                count: row.count,
                max_count: row.max_count,
            });
        }
    }

    let response = PackRewardObjectCountResponse { count_info };
    
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
    
    let (route, code) = PacketCodeType::PackRewardObjectCount.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}