use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EvilCastleRogueLikeRelicMixRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::evil::evil_castle_rogue_like_relic_mix;
use sqlx::SqlitePool;

#[put("EvilCastleRogueLikeRelicMix")]
async fn evil_castle_rogue_like_relic_mix_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<EvilCastleRogueLikeRelicMixRequest>("EvilCastleRogueLikeRelicMix", &body)
            .map_err(|e| {
                tracing::warn!("Failed to parse EvilCastleRogueLikeRelicMix: {}", e);
                actix_web::error::ErrorBadRequest("Invalid packet")
            })?;
    let response = evil_castle_rogue_like_relic_mix::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
