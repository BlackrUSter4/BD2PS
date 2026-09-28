use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, Notify, TalentSkillUpgradeRequest, TalentSkillUpgradeResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

/// Real item consumption (the response schema is just the consumed items echoed back — no
/// separate talent-upgrade-cost master table exists to validate against).
pub async fn handle(pool: &SqlitePool, uid: i64, req: TalentSkillUpgradeRequest) -> GameResponse {
    info!("Handling TalentSkillUpgradeRequest: {:?}", req);

    let mut item_info = Vec::new();
    for item in &req.item_info {
        if let (Some(id), Some(count)) = (item.id, item.count) {
            if item_info::consume(pool, uid, id, count).await.unwrap_or(false) {
                item_info.push(ItemDbInfo { id: Some(id), r#type: item.r#type, count: Some(count), ..Default::default() });
            }
        }
    }

    let response = TalentSkillUpgradeResponse { item_info };

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

    let (route, code) = PacketCodeType::TalentSkillUpgrade.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
