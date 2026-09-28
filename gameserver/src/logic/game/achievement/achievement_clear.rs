use bd2::prost::Message;
use bd2::proto::proto_net::{AchievementClearRequest, AchievementClearResponse, ItemDbInfo, Notify, RewardDbInfoBundle};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{achievement::achievement_info as db, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

/// Real claim-once tier reward against AchievementTable's real condition_value/reward arrays/
/// exp — each tier id is only granted if the account's real progress Value meets its real
/// condition and it's higher than the account's real MaxClearId so far.
pub async fn handle(pool: &SqlitePool, uid: i64, req: AchievementClearRequest) -> GameResponse {
    info!("Handling AchievementClearRequest: {:?}", req);

    let game_data = data::exceldb::get();
    let mut add_exp = 0;
    let mut item_infos = Vec::new();

    for clear in &req.clear_info {
        let Some(group_id) = clear.group_id else { continue };
        let Some(row) = db::get_by_group(pool, uid, group_id).await.ok().flatten() else { continue };
        let value = row.value.unwrap_or(0);
        let mut max_clear_id = row.max_clear_id.unwrap_or(0);

        for &id in &clear.id {
            if id <= max_clear_id {
                continue;
            }
            let Some(def) = game_data.achievementtable.get(id) else { continue };
            if (value as f32) < def.condition_value {
                continue;
            }

            add_exp += def.exp.unwrap_or(0);
            if let Some(ids) = &def.reward_item_id {
                for i in 0..ids.len() {
                    let item_id = ids[i];
                    let ty = def.reward_item_type.as_ref().and_then(|v| v.get(i).copied()).unwrap_or(1);
                    let count = def.reward_item_count.as_ref().and_then(|v| v.get(i).copied()).unwrap_or(1);
                    let _ = item_info::grant(pool, uid, item_id, ty, count).await;
                    item_infos.push(ItemDbInfo { id: Some(item_id), r#type: Some(ty), count: Some(count), ..Default::default() });
                }
            }
            max_clear_id = max_clear_id.max(id);
        }

        let _ = db::set_max_clear_id(pool, uid, group_id, max_clear_id).await;
    }

    let response = AchievementClearResponse {
        add_exp: Some(add_exp),
        reward_info_bundle: Some(RewardDbInfoBundle { item_info: item_infos, ..Default::default() }),
    };

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

    let (route, code) = PacketCodeType::AchievementClear.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
