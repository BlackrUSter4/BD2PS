use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FieldObjectPreviewRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::field::field_object_preview;
use sqlx::SqlitePool;

#[put("FieldObjectPreview")]
async fn field_object_preview_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<FieldObjectPreviewRequest>("FieldObjectPreview", &body).map_err(|e| {
            tracing::warn!("Failed to parse FieldObjectPreview: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = field_object_preview::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
