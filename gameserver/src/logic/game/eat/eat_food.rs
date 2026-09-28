use bd2::prost::Message;
use bd2::proto::proto_net::{CharDbInfo, EatFoodRequest, EatFoodResponse, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{char::char_info, item::item_info};
use sqlx::SqlitePool;
use tracing::info;

/// Real feed: consumes the real food items, heals the real character HP by FoodTable's real
/// recovery_point. No MaxHp stat exists anywhere in this project (confirmed in the Char
/// round), so the heal is uncapped rather than fabricating a cap. field_buff_info stays empty
/// — FoodBuffTable exists but no per-account active-buff tracking table does.
pub async fn handle(pool: &SqlitePool, uid: i64, req: EatFoodRequest) -> GameResponse {
    info!("Handling EatFoodRequest: {:?}", req);

    let mut char_info = Vec::new();
    if let Some(char_inven_index) = req.inven_index {
        let mut heal = 0i64;
        for item in &req.item_info {
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
            char_info.push(CharDbInfo {
                inven_index: row.inven_index,
                id: row.id,
                hp: row.hp,
                level: row.level,
                costume_id: row.costume_id,
                exp: row.exp,
                use_costume: row.use_costume,
                talent_level: row.talent_level,
                talent_exp: row.talent_exp,
                solidarity_reward: row.solidarity_reward,
                expiry_time: row.expiry_time,
                pictorialbook_info: vec![],
                connect_potential_costume: row.connect_potential_costume,
            });
        }
    }

    let response = EatFoodResponse { char_info, field_buff_info: vec![] };

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

    let (route, code) = PacketCodeType::EatFood.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
