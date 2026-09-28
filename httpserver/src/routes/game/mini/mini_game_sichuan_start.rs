use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameSichuanStartRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_sichuan_start;
use sqlx::SqlitePool;

#[put("MiniGameSichuanStart")]
async fn mini_game_sichuan_start_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MiniGameSichuanStartRequest>("MiniGameSichuanStart", &body).map_err(
        |e| {
            tracing::warn!("Failed to parse MiniGameSichuanStart: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        },
    )?;
    let response = mini_game_sichuan_start::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
