use actix_web::{put, web, HttpResponse, Result};
use bd2::proto::proto_net::MissionUpdateRequest;
use crypto::network::parse_packet;
use gameserver::logic::game::mission::mission_update;
use sqlx::SqlitePool;

#[put("MissionUpdate")]
async fn mission_update_handler(
    pool: web::Data<SqlitePool>,
    body: String,
    user_id: web::ReqData<i64>,
) -> Result<HttpResponse> {
    let uid = *user_id;
    let req = parse_packet::<MissionUpdateRequest>("MissionUpdate", &body).map_err(|e| {
        tracing::warn!("Failed to parse MissionUpdate: {}", e);
        actix_web::error::ErrorBadRequest("Invalid packet")
    })?;
    let response = mission_update::handle(&pool, uid, req).await;
    Ok(HttpResponse::Ok().json(response))
}
