use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniPuzzleOpenRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_puzzle_open;
use sqlx::SqlitePool;

#[put("MiniPuzzleOpen")]
async fn mini_puzzle_open_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MiniPuzzleOpenRequest>("MiniPuzzleOpen", &body).map_err(|e| {
        tracing::warn!("Failed to parse MiniPuzzleOpen: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = mini_puzzle_open::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
