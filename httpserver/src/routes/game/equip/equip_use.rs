use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EquipUseRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::equip::equip_use;
use sqlx::SqlitePool;

#[put("EquipUse")]
async fn equip_use_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EquipUseRequest>("EquipUse", &body).map_err(|e| {
        tracing::warn!("Failed to parse EquipUse: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = equip_use::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
