use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, PackJamEventRequest, PackJamEventResponse, Notify };
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

/// Real reward count/type from PackJamEventTable's single real config row. Judgment call:
/// the table has no reward_id field, so GOLD_ITEM_ID is used as the granted item — the
/// table's `insert_min`/`insert_max` fields look unrelated to this reward (more likely a
/// UI/RNG display range for something else) and aren't used here.
pub async fn handle(pool: &SqlitePool, uid: i64, req: PackJamEventRequest) -> GameResponse {
    info!("Handling PackJamEventRequest: {:?}", req);

    let pack_event_reward = data::exceldb::get()
        .packjameventtable
        .all()
        .first()
        .map(|cfg| (cfg.reward_type, cfg.reward_count));

    let mut reward = None;
    if let Some((ty, count)) = pack_event_reward {
        if count > 0 {
            let _ = item_info::grant(pool, uid, super::GOLD_ITEM_ID, ty, count).await;
            reward = Some(ItemDbInfo {
                id: Some(super::GOLD_ITEM_ID),
                r#type: Some(ty),
                count: Some(count),
                ..Default::default()
            });
        }
    }

    let response = PackJamEventResponse { pack_event_reward: reward };
    
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
    
    let (route, code) = PacketCodeType::PackJamEvent.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}