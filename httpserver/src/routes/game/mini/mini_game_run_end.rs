use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameRunEndRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_run_end;
use sqlx::SqlitePool;

#[put("MiniGameRunEnd")]
async fn mini_game_run_end_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MiniGameRunEndRequest>("MiniGameRunEnd", &body).map_err(|e| {
        tracing::warn!("Failed to parse MiniGameRunEnd: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = mini_game_run_end::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
