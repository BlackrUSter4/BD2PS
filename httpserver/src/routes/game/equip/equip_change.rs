use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EquipChangeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::equip::equip_change;
use sqlx::SqlitePool;

#[put("EquipChange")]
async fn equip_change_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EquipChangeRequest>("EquipChange", &body).map_err(|e| {
        tracing::warn!("Failed to parse EquipChange: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = equip_change::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
