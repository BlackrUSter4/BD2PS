use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::PassBuyRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::pass::pass_buy;
use sqlx::SqlitePool;

#[put("PassBuy")]
async fn pass_buy_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<PassBuyRequest>("PassBuy", &body).map_err(|e| {
        tracing::warn!("Failed to parse PassBuy: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = pass_buy::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
