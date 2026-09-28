use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameHopscotchInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_hopscotch_info;
use sqlx::SqlitePool;

#[put("MiniGameHopscotchInfo")]
async fn mini_game_hopscotch_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MiniGameHopscotchInfoRequest>("MiniGameHopscotchInfo", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse MiniGameHopscotchInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = mini_game_hopscotch_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
