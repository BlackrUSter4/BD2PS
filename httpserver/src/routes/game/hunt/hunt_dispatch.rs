use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::HuntDispatchRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::hunt::hunt_dispatch;
use sqlx::SqlitePool;

#[put("HuntDispatch")]
async fn hunt_dispatch_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<HuntDispatchRequest>("HuntDispatch", &body).map_err(|e| {
        tracing::warn!("Failed to parse HuntDispatch: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = hunt_dispatch::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
