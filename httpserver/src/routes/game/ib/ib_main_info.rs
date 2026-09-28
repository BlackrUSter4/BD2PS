use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::IbMainInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::ib::ib_main_info;
use sqlx::SqlitePool;

#[put("IbMainInfo")]
async fn ib_main_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<IbMainInfoRequest>("IbMainInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse IbMainInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = ib_main_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
