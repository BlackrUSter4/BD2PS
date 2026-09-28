use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::HuntDispatchEndRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::hunt::hunt_dispatch_end;
use sqlx::SqlitePool;

#[put("HuntDispatchEnd")]
async fn hunt_dispatch_end_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<HuntDispatchEndRequest>("HuntDispatchEnd", &body).map_err(|e| {
        tracing::warn!("Failed to parse HuntDispatchEnd: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = hunt_dispatch_end::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
