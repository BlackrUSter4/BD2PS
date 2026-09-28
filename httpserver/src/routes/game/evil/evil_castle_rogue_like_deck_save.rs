use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EvilCastleRogueLikeDeckSaveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::evil::evil_castle_rogue_like_deck_save;
use sqlx::SqlitePool;

#[put("EvilCastleRogueLikeDeckSave")]
async fn evil_castle_rogue_like_deck_save_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<EvilCastleRogueLikeDeckSaveRequest>("EvilCastleRogueLikeDeckSave", &body)
            .map_err(|e| {
                tracing::warn!("Failed to parse EvilCastleRogueLikeDeckSave: {}", e);
                actix_web::error::ErrorBadRequest("Invalid packet")
            })?;
    let response = evil_castle_rogue_like_deck_save::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
