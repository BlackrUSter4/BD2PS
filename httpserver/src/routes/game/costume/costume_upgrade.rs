use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CostumeUpgradeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::costume::costume_upgrade;
use sqlx::SqlitePool;

#[put("CostumeUpgrade")]
async fn costume_upgrade_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<CostumeUpgradeRequest>("CostumeUpgrade", &body).map_err(|e| {
        tracing::warn!("Failed to parse CostumeUpgrade: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = costume_upgrade::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
