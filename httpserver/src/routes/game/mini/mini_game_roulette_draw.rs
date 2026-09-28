use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameRouletteDrawRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_roulette_draw;
use sqlx::SqlitePool;

#[put("MiniGameRouletteDraw")]
async fn mini_game_roulette_draw_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MiniGameRouletteDrawRequest>("MiniGameRouletteDraw", &body).map_err(
        |e| {
            tracing::warn!("Failed to parse MiniGameRouletteDraw: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        },
    )?;
    let response = mini_game_roulette_draw::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
