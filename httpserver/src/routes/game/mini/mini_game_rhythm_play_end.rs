use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameRhythmPlayEndRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_rhythm_play_end;
use sqlx::SqlitePool;

#[put("MiniGameRhythmPlayEnd")]
async fn mini_game_rhythm_play_end_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MiniGameRhythmPlayEndRequest>("MiniGameRhythmPlayEnd", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse MiniGameRhythmPlayEnd: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = mini_game_rhythm_play_end::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
