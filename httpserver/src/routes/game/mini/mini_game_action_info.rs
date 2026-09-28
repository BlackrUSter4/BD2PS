use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameActionInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_action_info;
use sqlx::SqlitePool;

#[put("MiniGameActionInfo")]
async fn mini_game_action_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<MiniGameActionInfoRequest>("MiniGameActionInfo", &body).map_err(|e| {
            tracing::warn!("Failed to parse MiniGameActionInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = mini_game_action_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
