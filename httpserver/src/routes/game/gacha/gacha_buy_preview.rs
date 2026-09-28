use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::GachaBuyPreviewRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::gacha::gacha_buy_preview;
use sqlx::SqlitePool;

#[put("GachaBuyPreview")]
async fn gacha_buy_preview_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<GachaBuyPreviewRequest>("GachaBuyPreview", &body).map_err(|e| {
        tracing::warn!("Failed to parse GachaBuyPreview: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = gacha_buy_preview::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
