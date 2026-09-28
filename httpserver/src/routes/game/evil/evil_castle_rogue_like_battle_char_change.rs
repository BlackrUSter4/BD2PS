use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EvilCastleRogueLikeBattleCharChangeRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::evil::evil_castle_rogue_like_battle_char_change;
use sqlx::SqlitePool;

#[put("EvilCastleRogueLikeBattleCharChange")]
async fn evil_castle_rogue_like_battle_char_change_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EvilCastleRogueLikeBattleCharChangeRequest>(
        "EvilCastleRogueLikeBattleCharChange",
        &body,
    )
    .map_err(|e| {
        tracing::warn!("Failed to parse EvilCastleRogueLikeBattleCharChange: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = evil_castle_rogue_like_battle_char_change::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
