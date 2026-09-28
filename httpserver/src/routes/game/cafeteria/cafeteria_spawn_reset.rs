use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CafeteriaSpawnResetRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::cafeteria::cafeteria_spawn_reset;
use sqlx::SqlitePool;

#[put("CafeteriaSpawnReset")]
async fn cafeteria_spawn_reset_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<CafeteriaSpawnResetRequest>("CafeteriaSpawnReset", &body).map_err(|e| {
            tracing::warn!("Failed to parse CafeteriaSpawnReset: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = cafeteria_spawn_reset::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
