use bd2::prost::Message;
use bd2::proto::proto_net::{eat_food_auto_response::RecoveryCharInfo, EatFoodAutoRequest, EatFoodAutoResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{char::char_info, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

/// Same real feed logic as EatFood, batched over multiple characters.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EatFoodAutoRequest) -> GameResponse {
    info!("Handling EatFoodAutoRequest: {:?}", req);

    let mut recovery_char_info = Vec::new();
    for entry in &req.eat_food_info {
        let Some(char_inven_index) = entry.inven_index else { continue };

        let mut heal = 0i64;
        for item in &entry.item_info {
            if let (Some(id), Some(count)) = (item.id, item.count) {
                if item_info::consume(pool, uid, id, count).await.unwrap_or(false) {
                    if let Some(food) = data::exceldb::get().foodtable.get(id) {
                        heal += food.recovery_point as i64 * count as i64;
                    }
                }
            }
        }

        if heal > 0 {
            if let Ok(Some(row)) = char_info::get_by_inven_index(pool, uid, char_inven_index).await {
                let new_hp = row.hp.unwrap_or(0) + heal;
                let _ = char_info::set_hp_by_inven(pool, uid, char_inven_index, new_hp).await;
            }
        }

        if let Ok(Some(row)) = char_info::get_by_inven_index(pool, uid, char_inven_index).await {
            recovery_char_info.push(RecoveryCharInfo { char_inven_index: row.inven_index, hp: row.hp });
        }
    }

    let response = EatFoodAutoResponse { recovery_char_info };

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

    let (route, code) = PacketCodeType::EatFoodAuto.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
