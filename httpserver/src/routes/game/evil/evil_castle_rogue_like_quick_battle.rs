use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EvilCastleRogueLikeQuickBattleRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::evil::evil_castle_rogue_like_quick_battle;
use sqlx::SqlitePool;

#[put("EvilCastleRogueLikeQuickBattle")]
async fn evil_castle_rogue_like_quick_battle_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EvilCastleRogueLikeQuickBattleRequest>(
        "EvilCastleRogueLikeQuickBattle",
        &body,
    )
    .map_err(|e| {
        tracing::warn!("Failed to parse EvilCastleRogueLikeQuickBattle: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = evil_castle_rogue_like_quick_battle::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
