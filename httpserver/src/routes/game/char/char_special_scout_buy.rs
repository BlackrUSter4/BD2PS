use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::CharSpecialScoutBuyRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::char::char_special_scout_buy;
use sqlx::SqlitePool;

#[put("CharSpecialScoutBuy")]
async fn char_special_scout_buy_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req =
        parse_packet::<CharSpecialScoutBuyRequest>("CharSpecialScoutBuy", &body).map_err(|e| {
            tracing::warn!("Failed to parse CharSpecialScoutBuy: {}", e);
            actix_web::error::ErrorBadRequest("Invalid packet")
        })?;
    let response = char_special_scout_buy::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
