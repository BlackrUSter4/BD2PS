use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EquipUpgradeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::equip::equip_upgrade;
use sqlx::SqlitePool;

#[put("EquipUpgrade")]
async fn equip_upgrade_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EquipUpgradeRequest>("EquipUpgrade", &body).map_err(|e| {
        tracing::warn!("Failed to parse EquipUpgrade: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = equip_upgrade::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
