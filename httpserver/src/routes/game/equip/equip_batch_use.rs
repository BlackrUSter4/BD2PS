use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EquipBatchUseRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::equip::equip_batch_use;
use sqlx::SqlitePool;

#[put("EquipBatchUse")]
async fn equip_batch_use_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EquipBatchUseRequest>("EquipBatchUse", &body).map_err(|e| {
        tracing::warn!("Failed to parse EquipBatchUse: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = equip_batch_use::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
