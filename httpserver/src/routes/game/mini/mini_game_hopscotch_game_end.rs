use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameHopscotchGameEndRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_hopscotch_game_end;
use sqlx::SqlitePool;

#[put("MiniGameHopscotchGameEnd")]
async fn mini_game_hopscotch_game_end_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MiniGameHopscotchGameEndRequest>("MiniGameHopscotchGameEnd", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse MiniGameHopscotchGameEnd: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = mini_game_hopscotch_game_end::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
