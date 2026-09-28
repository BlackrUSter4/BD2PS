use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EvilCastleRogueLikeBattleEndPreviewRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::evil::evil_castle_rogue_like_battle_end_preview;
use sqlx::SqlitePool;

#[put("EvilCastleRogueLikeBattleEndPreview")]
async fn evil_castle_rogue_like_battle_end_preview_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<EvilCastleRogueLikeBattleEndPreviewRequest>(
        "EvilCastleRogueLikeBattleEndPreview",
        &body,
    )
    .map_err(|e| {
        tracing::warn!("Failed to parse EvilCastleRogueLikeBattleEndPreview: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = evil_castle_rogue_like_battle_end_preview::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
