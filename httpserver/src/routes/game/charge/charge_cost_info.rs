use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::ChargeCostInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::charge::charge_cost_info;
use sqlx::SqlitePool;

#[put("ChargeCostInfo")]
async fn charge_cost_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req = parse_packet::<ChargeCostInfoRequest>("ChargeCostInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse ChargeCostInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;

    let response = charge_cost_info::handle(&pool, uid, req).await;

    Ok(HttpResponse::Ok().json(response))
}
