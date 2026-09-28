use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EvilCastleRogueLikeBattleSkipRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::evil::evil_castle_rogue_like_battle_skip;
use sqlx::SqlitePool;

#[put("EvilCastleRogueLikeBattleSkip")]
async fn evil_castle_rogue_like_battle_skip_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EvilCastleRogueLikeBattleSkipRequest>(
        "EvilCastleRogueLikeBattleSkip",
        &body,
    )
    .map_err(|e| {
        tracing::warn!("Failed to parse EvilCastleRogueLikeBattleSkip: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = evil_castle_rogue_like_battle_skip::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
