use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::IbStageEndRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::ib::ib_stage_end;
use sqlx::SqlitePool;

#[put("IbStageEnd")]
async fn ib_stage_end_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<IbStageEndRequest>("IbStageEnd", &body).map_err(|e| {
        tracing::warn!("Failed to parse IbStageEnd: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = ib_stage_end::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
