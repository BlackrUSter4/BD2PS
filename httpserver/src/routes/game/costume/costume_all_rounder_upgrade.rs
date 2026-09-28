use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CostumeAllRounderUpgradeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::costume::costume_all_rounder_upgrade;
use sqlx::SqlitePool;

#[put("CostumeAllRounderUpgrade")]
async fn costume_all_rounder_upgrade_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CostumeAllRounderUpgradeRequest>("CostumeAllRounderUpgrade", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse CostumeAllRounderUpgrade: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = costume_all_rounder_upgrade::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
