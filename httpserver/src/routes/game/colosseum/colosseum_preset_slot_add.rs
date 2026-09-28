use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ColosseumPresetSlotAddRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::colosseum::colosseum_preset_slot_add;
use sqlx::SqlitePool;

#[put("ColosseumPresetSlotAdd")]
async fn colosseum_preset_slot_add_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ColosseumPresetSlotAddRequest>("ColosseumPresetSlotAdd", &body).map_err(|e| {
        tracing::warn!("Failed to parse ColosseumPresetSlotAdd: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = colosseum_preset_slot_add::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
