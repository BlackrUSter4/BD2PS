use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MiniGameHopscotchRankingInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mini::mini_game_hopscotch_ranking_info;
use sqlx::SqlitePool;

#[put("MiniGameHopscotchRankingInfo")]
async fn mini_game_hopscotch_ranking_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MiniGameHopscotchRankingInfoRequest>("MiniGameHopscotchRankingInfo", &body)
        .map_err(|e| {
            tracing::warn!("Failed to parse MiniGameHopscotchRankingInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = mini_game_hopscotch_ranking_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
