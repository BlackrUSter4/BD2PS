use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CafeteriaLevelUpRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::cafeteria::cafeteria_level_up;
use sqlx::SqlitePool;

#[put("CafeteriaLevelUp")]
async fn cafeteria_level_up_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CafeteriaLevelUpRequest>("CafeteriaLevelUp", &body).map_err(|e| {
        tracing::warn!("Failed to parse CafeteriaLevelUp: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = cafeteria_level_up::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
