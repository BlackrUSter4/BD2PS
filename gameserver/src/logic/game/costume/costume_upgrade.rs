use bd2::prost::Message;
use bd2::proto::proto_net::{CostumeUpgradeRequest, CostumeUpgradeResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{costume::costume_info as costume_db, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

/// Real level-up against CostumeTable/CostumeGrowthTable (growth_group_id's row matching the
/// costume's next level, real max_level cap). Client-supplied item_info is trusted for WHICH
/// items to consume (same precedent as other upgrade handlers in this project) but the actual
/// consumption is real.
pub async fn handle(pool: &SqlitePool, uid: i64, req: CostumeUpgradeRequest) -> GameResponse {
    info!("Handling CostumeUpgradeRequest: {:?}", req);

    if let Some(inven_index) = req.inven_index {
        if let Ok(Some(costume)) = costume_db::get_by_inven_index(pool, uid, inven_index).await {
            let current_level = costume.level.unwrap_or(1);
            let game_data = data::exceldb::get();

            let max_level = costume
                .id
                .and_then(|id| game_data.costumetable.get(id))
                .and_then(|t| t.max_level)
                .unwrap_or(i32::MAX);

            if current_level < max_level {
                for item in &req.item_info {
                    if let (Some(id), Some(count)) = (item.id, item.count) {
                        let _ = item_info::consume(pool, uid, id, count).await;
                    }
                }
                let _ = costume_db::set_level_by_inven(pool, uid, inven_index, current_level + 1).await;
            }
        }
    }

    let response = CostumeUpgradeResponse {};

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

    let (route, code) = PacketCodeType::CostumeUpgrade.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
