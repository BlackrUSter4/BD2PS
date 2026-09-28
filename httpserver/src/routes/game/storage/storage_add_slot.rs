use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::StorageAddSlotRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::storage::storage_add_slot;
use sqlx::SqlitePool;

#[put("StorageAddSlot")]
async fn storage_add_slot_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<StorageAddSlotRequest>("StorageAddSlot", &body).map_err(|e| {
        tracing::warn!("Failed to parse StorageAddSlot: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = storage_add_slot::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
