use bd2::prost::Message;
use bd2::proto::proto_net::{CookingRequest, CookingResponse, ItemDbInfo, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::item::item_info;
use sqlx::SqlitePool;
use tracing::info;

/// Real cooking against CookingTable's real material cost / result item / talent_level (same
/// talent-exp-per-craft formula as Alchemy).
pub async fn handle(pool: &SqlitePool, uid: i64, req: CookingRequest) -> GameResponse {
    info!("Handling CookingRequest: {:?}", req);

    let mut item_infos = Vec::new();
    let mut add_talent_exp = 0;

    if let Some(recipe_id) = req.recipe_id {
        let count = req.cooking_count.filter(|&c| c > 0).unwrap_or(1);
        if let Some(def) = data::exceldb::get().cookingtable.get(recipe_id).cloned() {
            for i in 0..def.material_item_id.len() {
                let id = def.material_item_id[i];
                let per = *def.material_item_count.get(i).unwrap_or(&1);
                let _ = item_info::consume(pool, uid, id, per * count).await;
            }

            let result_count = def.result_item_count.max(1) * count;
            let _ = item_info::grant(pool, uid, def.result_item_id, 1, result_count).await;
            item_infos.push(ItemDbInfo { id: Some(def.result_item_id), r#type: Some(1), count: Some(result_count), ..Default::default() });
            add_talent_exp = def.talent_level * count;
        }
    }

    let response = CookingResponse { item_info: item_infos, add_talent_exp: Some(add_talent_exp) };

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

    let (route, code) = PacketCodeType::Cooking.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
