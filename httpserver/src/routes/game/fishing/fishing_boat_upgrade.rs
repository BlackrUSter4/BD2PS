use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::FishingBoatUpgradeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::fishing::fishing_boat_upgrade;
use sqlx::SqlitePool;

#[put("FishingBoatUpgrade")]
async fn fishing_boat_upgrade_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<FishingBoatUpgradeRequest>("FishingBoatUpgrade", &body).map_err(|e| {
        tracing::warn!("Failed to parse FishingBoatUpgrade: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = fishing_boat_upgrade::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
