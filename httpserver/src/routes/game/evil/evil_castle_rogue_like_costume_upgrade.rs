use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EvilCastleRogueLikeCostumeUpgradeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::evil::evil_castle_rogue_like_costume_upgrade;
use sqlx::SqlitePool;

#[put("EvilCastleRogueLikeCostumeUpgrade")]
async fn evil_castle_rogue_like_costume_upgrade_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EvilCastleRogueLikeCostumeUpgradeRequest>(
        "EvilCastleRogueLikeCostumeUpgrade",
        &body,
    )
    .map_err(|e| {
        tracing::warn!("Failed to parse EvilCastleRogueLikeCostumeUpgrade: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = evil_castle_rogue_like_costume_upgrade::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
