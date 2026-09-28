use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::WaypointInfoRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::waypoint::waypoint_info;
use sqlx::SqlitePool;

#[put("WaypointInfo")]
async fn waypoint_info_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;

    let req = parse_packet::<WaypointInfoRequest>("WaypointInfo", &body).map_err(|e| {
        tracing::warn!("Failed to parse WaypointInfo: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;

    let response = waypoint_info::handle(&pool, uid, req).await;

    Ok(HttpResponse::Ok().json(response))
}
