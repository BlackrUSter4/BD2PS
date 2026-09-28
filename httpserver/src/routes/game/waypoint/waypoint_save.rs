use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::WaypointSaveRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::waypoint::waypoint_save;
use sqlx::SqlitePool;

#[put("WaypointSave")]
async fn waypoint_save_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<WaypointSaveRequest>("WaypointSave", &body).map_err(|e| {
        tracing::warn!("Failed to parse WaypointSave: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = waypoint_save::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
