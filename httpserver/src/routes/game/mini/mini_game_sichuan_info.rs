use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameSichuanInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_sichuan_info;
use sqlx::SqlitePool;

#[put("MiniGameSichuanInfo")]
async fn mini_game_sichuan_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<MiniGameSichuanInfoRequest>("MiniGameSichuanInfo", &body).map_err(|e| {
            tracing::warn!("Failed to parse MiniGameSichuanInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = mini_game_sichuan_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
