use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PackBuyRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pack::pack_buy;
use sqlx::SqlitePool;

#[put("PackBuy")]
async fn pack_buy_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PackBuyRequest>("PackBuy", &body).map_err(|e| {
        tracing::warn!("Failed to parse PackBuy: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = pack_buy::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
