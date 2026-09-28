use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameDefenseMatchingRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_defense_matching;
use sqlx::SqlitePool;

#[put("MiniGameDefenseMatching")]
async fn mini_game_defense_matching_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MiniGameDefenseMatchingRequest>("MiniGameDefenseMatching", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse MiniGameDefenseMatching: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = mini_game_defense_matching::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
