use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EquipRankUpgradeBuyRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::equip::equip_rank_upgrade_buy;
use sqlx::SqlitePool;

#[put("EquipRankUpgradeBuy")]
async fn equip_rank_upgrade_buy_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EquipRankUpgradeBuyRequest>("EquipRankUpgradeBuy", &body).map_err(|e| {
        tracing::warn!("Failed to parse EquipRankUpgradeBuy: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = equip_rank_upgrade_buy::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
