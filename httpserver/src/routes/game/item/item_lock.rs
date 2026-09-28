use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ItemLockRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::item::item_lock;
use sqlx::SqlitePool;

#[put("ItemLock")]
async fn item_lock_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ItemLockRequest>("ItemLock", &body).map_err(|e| {
        tracing::warn!("Failed to parse ItemLock: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = item_lock::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
