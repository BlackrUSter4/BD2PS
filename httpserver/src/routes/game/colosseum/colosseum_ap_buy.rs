use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ColosseumApBuyRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::colosseum::colosseum_ap_buy;
use sqlx::SqlitePool;

#[put("ColosseumApBuy")]
async fn colosseum_ap_buy_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<ColosseumApBuyRequest>("ColosseumApBuy", &body).map_err(|e| {
        tracing::warn!("Failed to parse ColosseumApBuy: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = colosseum_ap_buy::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
