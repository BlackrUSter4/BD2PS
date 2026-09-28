use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EvilCastleRogueLikeEventChoiceRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::evil::evil_castle_rogue_like_event_choice;
use sqlx::SqlitePool;

#[put("EvilCastleRogueLikeEventChoice")]
async fn evil_castle_rogue_like_event_choice_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EvilCastleRogueLikeEventChoiceRequest>(
        "EvilCastleRogueLikeEventChoice",
        &body,
    )
    .map_err(|e| {
        tracing::warn!("Failed to parse EvilCastleRogueLikeEventChoice: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = evil_castle_rogue_like_event_choice::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
