use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EquipLockRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::equip::equip_lock;
use sqlx::SqlitePool;

#[put("EquipLock")]
async fn equip_lock_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EquipLockRequest>("EquipLock", &body).map_err(|e| {
        tracing::warn!("Failed to parse EquipLock: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = equip_lock::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
