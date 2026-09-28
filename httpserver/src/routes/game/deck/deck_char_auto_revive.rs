use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::DeckCharAutoReviveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::deck::deck_char_auto_revive;
use sqlx::SqlitePool;

#[put("DeckCharAutoRevive")]
async fn deck_char_auto_revive_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<DeckCharAutoReviveRequest>("DeckCharAutoRevive", &body).map_err(|e| {
            tracing::warn!("Failed to parse DeckCharAutoRevive: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = deck_char_auto_revive::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
