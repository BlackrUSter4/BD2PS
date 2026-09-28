use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EquipBreakRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::equip::equip_break;
use sqlx::SqlitePool;

#[put("EquipBreak")]
async fn equip_break_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EquipBreakRequest>("EquipBreak", &body).map_err(|e| {
        tracing::warn!("Failed to parse EquipBreak: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = equip_break::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
