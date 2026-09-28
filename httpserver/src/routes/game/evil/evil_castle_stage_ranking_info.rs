use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::EvilCastleStageRankingInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::evil::evil_castle_stage_ranking_info;
use sqlx::SqlitePool;

#[put("EvilCastleStageRankingInfo")]
async fn evil_castle_stage_ranking_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<EvilCastleStageRankingInfoRequest>("EvilCastleStageRankingInfo", &body)
            .map_err(|e| {
                tracing::warn!("Failed to parse EvilCastleStageRankingInfo: {}", e);
                actix_web::error::ErrorBadRequest("Invalid packet")
            })?;
    let response = evil_castle_stage_ranking_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
