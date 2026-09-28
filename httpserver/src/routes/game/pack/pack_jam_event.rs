use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PackJamEventRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pack::pack_jam_event;
use sqlx::SqlitePool;

#[put("PackJamEvent")]
async fn pack_jam_event_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PackJamEventRequest>("PackJamEvent", &body).map_err(|e| {
        tracing::warn!("Failed to parse PackJamEvent: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = pack_jam_event::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
