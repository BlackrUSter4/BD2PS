use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EquipUpgradeToBreakAutoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::equip::equip_upgrade_to_break_auto;
use sqlx::SqlitePool;

#[put("EquipUpgradeToBreakAuto")]
async fn equip_upgrade_to_break_auto_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<EquipUpgradeToBreakAutoRequest>("EquipUpgradeToBreakAuto", &body).map_err(|e| {
            tracing::warn!("Failed to parse EquipUpgradeToBreakAuto: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = equip_upgrade_to_break_auto::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
