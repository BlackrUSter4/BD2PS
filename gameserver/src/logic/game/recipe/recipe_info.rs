use bd2::prost::Message;
use bd2::proto::proto_net::{Notify, RecipeInfoRequest, RecipeInfoResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use database::db::recipe::recipe_info as recipe_db;
use sqlx::SqlitePool;
use tracing::info;

/// Real per-account unlocked-recipe list (was hardcoded to a single fake id before).
pub async fn handle(pool: &SqlitePool, uid: i64, req: RecipeInfoRequest) -> GameResponse {
    info!("Handling RecipeInfoRequest: {:?}", req);

    let recipe_id = recipe_db::get_recipe_info(pool, uid)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|r| r.recipe_id)
        .collect();

    let response = RecipeInfoResponse {
        recipe_id: recipe_id,
        ..Default::default()
    };

    let resp_bytes = response.encode_to_vec();

    let notify = Notify {
        achievement_update_info: vec![],
        mission_update_info: vec![],
        event_mission_update_info: vec![],
        active_login_event: vec![1, 2, 628, 629],
        active_contents_info: vec![],
        ll_type: Some("".to_string()),
        is_purchasing_disabled: Some(false),
        maintenance_start_date: Some(1688646600000),
        notice_last_seq: Some(8909),
        ..Default::default()
    };

    let (route, code) = PacketCodeType::RecipeInfo.info();
    GameResponse::success(route, &resp_bytes, code).with_notify(&notify)
}
