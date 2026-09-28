use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::InvenAddSlotRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::inven::inven_add_slot;
use sqlx::SqlitePool;

#[put("InvenAddSlot")]
async fn inven_add_slot_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<InvenAddSlotRequest>("InvenAddSlot", &body).map_err(|e| {
        tracing::warn!("Failed to parse InvenAddSlot: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = inven_add_slot::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
