use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::UpdateAgeGateRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::age_gate::update_age_gate;
use sqlx::SqlitePool;

#[put("UpdateAgeGate")]
async fn update_age_gate_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<UpdateAgeGateRequest>("UpdateAgeGate", &body).map_err(|e| {
        tracing::warn!("Failed to parse UpdateAgeGate: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = update_age_gate::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
