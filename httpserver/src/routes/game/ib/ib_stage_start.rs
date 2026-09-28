use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::IbStageStartRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::ib::ib_stage_start;
use sqlx::SqlitePool;

#[put("IbStageStart")]
async fn ib_stage_start_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<IbStageStartRequest>("IbStageStart", &body).map_err(|e| {
        tracing::warn!("Failed to parse IbStageStart: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = ib_stage_start::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
