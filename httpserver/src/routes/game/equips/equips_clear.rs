use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EquipsClearRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::equips::equips_clear;
use sqlx::SqlitePool;

#[put("EquipsClear")]
async fn equips_clear_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EquipsClearRequest>("EquipsClear", &body).map_err(|e| {
        tracing::warn!("Failed to parse EquipsClear: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = equips_clear::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
