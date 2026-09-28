use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PresetAddSlotRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::preset::preset_add_slot;
use sqlx::SqlitePool;

#[put("PresetAddSlot")]
async fn preset_add_slot_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PresetAddSlotRequest>("PresetAddSlot", &body).map_err(|e| {
        tracing::warn!("Failed to parse PresetAddSlot: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = preset_add_slot::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
