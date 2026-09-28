use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::TrapDamageRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::trap::trap_damage;
use sqlx::SqlitePool;

#[put("TrapDamage")]
async fn trap_damage_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<TrapDamageRequest>("TrapDamage", &body).map_err(|e| {
        tracing::warn!("Failed to parse TrapDamage: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = trap_damage::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
