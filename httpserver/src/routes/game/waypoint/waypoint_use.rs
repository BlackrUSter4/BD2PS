use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::WaypointUseRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::waypoint::waypoint_use;
use sqlx::SqlitePool;

#[put("WaypointUse")]
async fn waypoint_use_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<WaypointUseRequest>("WaypointUse", &body).map_err(|e| {
        tracing::warn!("Failed to parse WaypointUse: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = waypoint_use::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
