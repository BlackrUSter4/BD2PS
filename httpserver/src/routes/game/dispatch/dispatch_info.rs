use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::DispatchInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::dispatch::dispatch_info;
use sqlx::SqlitePool;

#[put("DispatchInfo")]
async fn dispatch_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<DispatchInfoRequest>("DispatchInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse DispatchInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = dispatch_info::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
