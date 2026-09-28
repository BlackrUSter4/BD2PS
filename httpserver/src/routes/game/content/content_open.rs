use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ContentOpenRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::content::content_open;
use sqlx::SqlitePool;

#[put("ContentOpen")]
async fn content_open_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ContentOpenRequest>("ContentOpen", &body).map_err(|e| {
        tracing::warn!("Failed to parse ContentOpen: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = content_open::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
