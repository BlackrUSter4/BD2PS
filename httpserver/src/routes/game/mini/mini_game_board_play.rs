use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameBoardPlayRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_board_play;
use sqlx::SqlitePool;

#[put("MiniGameBoardPlay")]
async fn mini_game_board_play_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<MiniGameBoardPlayRequest>("MiniGameBoardPlay", &body).map_err(|e| {
            tracing::warn!("Failed to parse MiniGameBoardPlay: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = mini_game_board_play::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
