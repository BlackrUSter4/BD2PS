use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniPuzzleRenewRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_puzzle_renew;
use sqlx::SqlitePool;

#[put("MiniPuzzleRenew")]
async fn mini_puzzle_renew_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MiniPuzzleRenewRequest>("MiniPuzzleRenew", &body).map_err(|e| {
        tracing::warn!("Failed to parse MiniPuzzleRenew: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = mini_puzzle_renew::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
