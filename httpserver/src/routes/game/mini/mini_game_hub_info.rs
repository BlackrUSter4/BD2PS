use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameHubInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_hub_info;
use sqlx::SqlitePool;

#[put("MiniGameHubInfo")]
async fn mini_game_hub_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MiniGameHubInfoRequest>("MiniGameHubInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse MiniGameHubInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = mini_game_hub_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
