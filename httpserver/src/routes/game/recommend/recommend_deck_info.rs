use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::RecommendDeckInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::recommend::recommend_deck_info;
use sqlx::SqlitePool;

#[put("RecommendDeckInfo")]
async fn recommend_deck_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<RecommendDeckInfoRequest>("RecommendDeckInfo", &body).map_err(|e| {
            tracing::warn!("Failed to parse RecommendDeckInfo: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = recommend_deck_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
