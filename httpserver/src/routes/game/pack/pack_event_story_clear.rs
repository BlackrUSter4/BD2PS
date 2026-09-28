use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PackEventStoryClearRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pack::pack_event_story_clear;
use sqlx::SqlitePool;

#[put("PackEventStoryClear")]
async fn pack_event_story_clear_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<PackEventStoryClearRequest>("PackEventStoryClear", &body).map_err(|e| {
            tracing::warn!("Failed to parse PackEventStoryClear: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = pack_event_story_clear::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
