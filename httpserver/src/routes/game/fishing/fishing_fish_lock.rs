use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FishingFishLockRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::fishing::fishing_fish_lock;
use sqlx::SqlitePool;

#[put("FishingFishLock")]
async fn fishing_fish_lock_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FishingFishLockRequest>("FishingFishLock", &body).map_err(|e| {
        tracing::warn!("Failed to parse FishingFishLock: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = fishing_fish_lock::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
