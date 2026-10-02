use bd2::prost::Message;
use bd2::proto::proto_net::{ItemDbInfo, Notify, TalentSkillUpgradeRequest, TalentSkillUpgradeResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{char::char_info, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

/// Real item consumption plus a real TalentLevel increment — cross-checked against the
/// reference server's `GameTalentServer.TalentSkillUpgrade`, which is exactly this: no cost
/// table to validate against (confirmed, matches the original claim here), but it does
/// unconditionally bump the character's own CharInfo.TalentLevel by 1. The earlier version
/// consumed items but never persisted the level-up itself.
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

    if let Some(inven_index) = req.inven_index {
        if let Ok(Some(char_row)) = char_info::get_by_inven_index(pool, uid, inven_index).await {
            let new_level = char_row.talent_level.unwrap_or(0) + 1;
            let _ = char_info::set_talent_level_exp(pool, uid, inven_index, new_level, char_row.talent_exp.unwrap_or(0)).await;
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
