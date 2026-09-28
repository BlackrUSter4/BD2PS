use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::TotalWarDeckPresetSlotAddRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::total::total_war_deck_preset_slot_add;
use sqlx::SqlitePool;

#[put("TotalWarDeckPresetSlotAdd")]
async fn total_war_deck_preset_slot_add_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<TotalWarDeckPresetSlotAddRequest>("TotalWarDeckPresetSlotAdd", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse TotalWarDeckPresetSlotAdd: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = total_war_deck_preset_slot_add::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
