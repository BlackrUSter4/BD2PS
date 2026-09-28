use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EvilCastleRogueLikeEventSelectRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::evil::evil_castle_rogue_like_event_select;
use sqlx::SqlitePool;

#[put("EvilCastleRogueLikeEventSelect")]
async fn evil_castle_rogue_like_event_select_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EvilCastleRogueLikeEventSelectRequest>(
        "EvilCastleRogueLikeEventSelect",
        &body,
    )
    .map_err(|e| {
        tracing::warn!("Failed to parse EvilCastleRogueLikeEventSelect: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = evil_castle_rogue_like_event_select::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
