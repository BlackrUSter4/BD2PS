use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::DeckInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::deck::deck_info;
use sqlx::SqlitePool;

#[put("DeckInfo")]
async fn deck_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<DeckInfoRequest>("DeckInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse DeckInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = deck_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
