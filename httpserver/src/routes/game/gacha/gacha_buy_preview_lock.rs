use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GachaBuyPreviewLockRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::gacha::gacha_buy_preview_lock;
use sqlx::SqlitePool;

#[put("GachaBuyPreviewLock")]
async fn gacha_buy_preview_lock_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<GachaBuyPreviewLockRequest>("GachaBuyPreviewLock", &body).map_err(|e| {
            tracing::warn!("Failed to parse GachaBuyPreviewLock: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = gacha_buy_preview_lock::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
