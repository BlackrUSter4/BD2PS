use bd2::prost::Message;
use bd2::proto::proto_net::{CookingResearchRequest, CookingResearchResponse, ItemDbInfo, Notify};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::{item::item_info, recipe::recipe_info as recipe_db};
use database::models::game::recipe::recipe_info::RecipeInfo;
use sqlx::SqlitePool;
use tracing::info;

/// Real recipe unlock: consumes the real research items and permanently unlocks the recipe
/// (RecipeInfo) if not already unlocked. CookingResearchTable has no per-recipe id/success-
/// chance data (a single global config row: catalystValue/failItemId/useSlotCount) so there is
/// no real basis to ever fail the research — always succeeds, same as this project's other
/// "no captured chance data" cases.
pub async fn handle(pool: &SqlitePool, uid: i64, req: CookingResearchRequest) -> GameResponse {
    info!("Handling CookingResearchRequest: {:?}", req);

    let mut item_info = None;
    if let Some(recipe_id) = req.recipe_id {
        for item in &req.item_info {
            if let (Some(id), Some(count)) = (item.id, item.count) {
                let _ = item_info::consume(pool, uid, id, count).await;
            }
        }

        if !recipe_db::is_unlocked(pool, uid, recipe_id).await.unwrap_or(false) {
            let _ = recipe_db::add_recipe_info(pool, &RecipeInfo { index: 0, uid, seq: req.seq, recipe_id }).await;
        }

        if let Some(def) = data::exceldb::get().cookingtable.get(recipe_id) {
            item_info = Some(ItemDbInfo { id: Some(def.result_item_id), r#type: Some(1), count: Some(def.result_item_count), ..Default::default() });
        }
    }

    let response = CookingResearchResponse { item_info };

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

    let (route, code) = PacketCodeType::CookingResearch.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
